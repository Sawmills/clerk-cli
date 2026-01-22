import { getPreferenceValues } from "@raycast/api";
import { getCurrentInstance, ClerkInstance } from "./instances";

const BASE_URL = "https://api.clerk.com/v1";

// Preferences interface
interface Preferences {
  clerkApiKey: string;
  clerkFrontendApi?: string;
}

let cachedInstance: ClerkInstance | undefined | null = null;

export async function loadCurrentInstance(): Promise<ClerkInstance | undefined> {
  cachedInstance = await getCurrentInstance();
  return cachedInstance;
}

export function getCachedInstance(): ClerkInstance | undefined | null {
  return cachedInstance;
}

function resolveFrontendApi(): string {
  if (cachedInstance?.frontendApi) {
    return cachedInstance.frontendApi;
  }

  const preferences = getPreferenceValues<Preferences>();
  if (preferences.clerkFrontendApi) {
    return preferences.clerkFrontendApi;
  }

  // Fallback to the primary frontend API domain if not provided
  return "https://clerk.sawmills.ai";
}

// Response type interfaces
export interface EmailAddress {
  id: string;
  email_address: string;
}

export interface User {
  id: string;
  first_name: string | null;
  last_name: string | null;
  email_addresses: EmailAddress[];
  primary_email_address_id: string | null;
  last_sign_in_at: number | null;
  created_at: number;
}

export interface Organization {
  id: string;
  name: string;
  slug: string | null;
  members_count: number | null;
  created_at: number;
}

export interface PublicUserData {
  user_id: string;
  first_name: string | null;
  last_name: string | null;
  identifier: string | null;
}

export interface Membership {
  id: string;
  role: string;
  public_user_data: PublicUserData;
}

export interface SignInToken {
  id: string;
  url: string;
  status: string;
}

export interface JwtTemplate {
  id: string;
  name: string;
  lifetime: number;
}

export interface Session {
  id: string;
  user_id: string;
  status: string;
  last_active_organization_id: string | null;
}

// Error response interface
interface ClerkErrorDetail {
  message: string;
  code: string;
}

interface ClerkError {
  errors: ClerkErrorDetail[];
}

// Response wrapper interfaces
interface OrganizationsResponse {
  data: Organization[];
}

interface MembershipsResponse {
  data: Membership[];
}

interface SessionTokenResponse {
  jwt: string;
}

// API Client class
export class ClerkAPI {
  private apiKey: string;
  private baseUrl = BASE_URL;
  private frontendApi?: string;

  constructor(apiKey: string, frontendApi?: string) {
    this.apiKey = apiKey;
    this.frontendApi = frontendApi;
  }

  private async request<T>(endpoint: string, options?: RequestInit): Promise<T> {
    const url = `${this.baseUrl}${endpoint}`;
    const headers = {
      Authorization: `Bearer ${this.apiKey}`,
      "Content-Type": "application/json",
      ...options?.headers,
    };

    const response = await fetch(url, {
      ...options,
      headers,
    });

    if (!response.ok) {
      let errorMessage = "Unknown error";
      try {
        const clerkError: ClerkError = await response.json();
        errorMessage = clerkError.errors[0]?.message || errorMessage;
      } catch {
        errorMessage = `HTTP ${response.status}: ${response.statusText}`;
      }
      throw new Error(errorMessage);
    }

    return response.json();
  }

  async listOrganizations(limit = 10): Promise<Organization[]> {
    const response = await this.request<OrganizationsResponse>(
      `/organizations?limit=${limit}&order_by=-created_at`,
    );
    return response.data;
  }

  async listUsers(query?: string, limit = 10): Promise<User[]> {
    let endpoint = `/users?limit=${limit}&order_by=-created_at`;
    if (query) {
      endpoint += `&query=${encodeURIComponent(query)}`;
    }
    return this.request<User[]>(endpoint);
  }

  async getOrganization(id: string): Promise<Organization> {
    return this.request<Organization>(`/organizations/${id}`);
  }

  async listOrgMembers(orgId: string): Promise<Membership[]> {
    const response = await this.request<MembershipsResponse>(`/organizations/${orgId}/memberships?limit=100`);
    return response.data;
  }

  async createSignInToken(userId: string): Promise<SignInToken> {
    return this.request<SignInToken>("/sign_in_tokens", {
      method: "POST",
      body: JSON.stringify({
        user_id: userId,
        expires_in_seconds: 3600,
      }),
    });
  }

  async listJwtTemplates(): Promise<JwtTemplate[]> {
    return this.request<JwtTemplate[]>("/jwt_templates");
  }

  async createUserJwt(
    userId: string,
    templateName: string,
    orgId?: string,
    userEmail?: string,
  ): Promise<{ token: string }> {
    if (orgId) {
      if (!userEmail) {
        throw new Error("User email is required to generate a JWT for a specific organization.");
      }

      const frontendApi = this.frontendApi ?? resolveFrontendApi();

      const ticket = await this.request<{ token: string }>("/sign_in_tokens", {
        method: "POST",
        body: JSON.stringify({ user_id: userId, expires_in_seconds: 300 }),
      });

      const { signInId, authHeader: attemptAuth } = await this.startSignInAttempt(frontendApi, userEmail);
      const { sessionId, authHeader } = await this.completeTicketFirstFactor(
        frontendApi,
        signInId,
        attemptAuth,
        ticket.token,
      );

      await this.touchSession(frontendApi, sessionId, orgId, authHeader);
      const token = await this.mintSessionToken(frontendApi, sessionId, orgId, templateName, authHeader);
      return { token };
    }

    const sessions = await this.request<Session[]>(`/sessions?user_id=${userId}&status=active`);

    if (sessions.length === 0) {
      throw new Error("No active sessions found for user");
    }

    const session = sessions[0];
    const response = await this.request<SessionTokenResponse>(`/sessions/${session.id}/tokens/${templateName}`, {
      method: "POST",
    });

    return { token: response.jwt };
  }

  private async startSignInAttempt(frontendApi: string, email: string): Promise<{ signInId: string; authHeader: string }> {
    const body = new URLSearchParams({ identifier: email });
    const resp = await fetch(`${frontendApi}/v1/client/sign_ins`, {
      method: "POST",
      headers: { "Content-Type": "application/x-www-form-urlencoded" },
      body,
    });

    if (!resp.ok) {
      const text = await resp.text();
      throw new Error(`Sign-in start failed (${resp.status}): ${text}`);
    }

    const authHeader = resp.headers.get("authorization") || resp.headers.get("Authorization");
    if (!authHeader) {
      throw new Error("Authorization header missing from sign-in start response");
    }

    const data = await resp.json();
    const signInId = data.response?.id || data.id;
    if (!signInId) {
      throw new Error("Sign-in start did not return an id");
    }

    return { signInId, authHeader: authHeader.startsWith("Bearer") ? authHeader : `Bearer ${authHeader}` };
  }

  private async completeTicketFirstFactor(
    frontendApi: string,
    signInId: string,
    authHeader: string,
    ticket: string,
  ): Promise<{ sessionId: string; authHeader: string }> {
    const body = new URLSearchParams({ strategy: "ticket", ticket });
    const resp = await fetch(`${frontendApi}/v1/client/sign_ins/${signInId}/attempt_first_factor`, {
      method: "POST",
      headers: {
        "Content-Type": "application/x-www-form-urlencoded",
        Authorization: authHeader,
      },
      body,
    });

    if (!resp.ok) {
      const text = await resp.text();
      throw new Error(`Ticket factor failed (${resp.status}): ${text}`);
    }

    const nextAuth = resp.headers.get("authorization") || resp.headers.get("Authorization");
    if (!nextAuth) {
      throw new Error("Authorization header missing from ticket factor response");
    }

    const data = await resp.json();
    const sessionId = data.response?.created_session_id || data.created_session_id;
    if (!sessionId) {
      throw new Error(`Ticket flow did not yield a session id (status: ${data.response?.status || data.status})`);
    }

    return { sessionId, authHeader: nextAuth.startsWith("Bearer") ? nextAuth : `Bearer ${nextAuth}` };
  }

  private async touchSession(
    frontendApi: string,
    sessionId: string,
    orgId: string,
    authHeader: string,
  ): Promise<void> {
    const body = new URLSearchParams({ active_organization_id: orgId });
    const resp = await fetch(`${frontendApi}/v1/client/sessions/${sessionId}/touch`, {
      method: "POST",
      headers: {
        "Content-Type": "application/x-www-form-urlencoded",
        Authorization: authHeader,
      },
      body,
    });

    if (!resp.ok) {
      const text = await resp.text();
      throw new Error(`Session touch failed (${resp.status}): ${text}`);
    }
  }

  private async mintSessionToken(
    frontendApi: string,
    sessionId: string,
    orgId: string,
    templateName: string,
    authHeader: string,
  ): Promise<string> {
    const body = new URLSearchParams({ organization_id: orgId });
    let path = `${frontendApi}/v1/client/sessions/${sessionId}/tokens`;
    if (templateName) {
      path += `/${templateName}`;
    }

    const resp = await fetch(path, {
      method: "POST",
      headers: {
        "Content-Type": "application/x-www-form-urlencoded",
        Authorization: authHeader,
      },
      body,
    });

    if (!resp.ok) {
      const text = await resp.text();
      throw new Error(`Session token mint failed (${resp.status}): ${text}`);
    }

    const data = await resp.json();
    const token = data.jwt || data.token;
    if (!token) {
      throw new Error("Empty JWT returned");
    }
    return token;
  }
}

// Helper function to get a configured client instance
export function getClerkClient(): ClerkAPI {
  if (cachedInstance) {
    return new ClerkAPI(cachedInstance.apiKey, cachedInstance.frontendApi);
  }

  const preferences = getPreferenceValues<Preferences>();

  if (!preferences.clerkApiKey) {
    throw new Error("Clerk API key is required. Please configure it in extension preferences or add an instance.");
  }

  return new ClerkAPI(preferences.clerkApiKey, preferences.clerkFrontendApi);
}

export function getClerkClientForInstance(instance: ClerkInstance): ClerkAPI {
  return new ClerkAPI(instance.apiKey, instance.frontendApi);
}

// Helper functions for display
export function getUserDisplayName(user: User): string {
  if (user.first_name && user.last_name) {
    return `${user.first_name} ${user.last_name}`;
  }
  if (user.first_name) {
    return user.first_name;
  }
  if (user.last_name) {
    return user.last_name;
  }
  return "";
}

export function getUserPrimaryEmail(user: User): string | null {
  if (user.primary_email_address_id) {
    const primaryEmail = user.email_addresses.find((e) => e.id === user.primary_email_address_id);
    if (primaryEmail) {
      return primaryEmail.email_address;
    }
  }
  // Fallback to first email
  return user.email_addresses[0]?.email_address || null;
}

export function getMemberDisplayName(member: Membership): string {
  const { first_name, last_name } = member.public_user_data;
  if (first_name && last_name) {
    return `${first_name} ${last_name}`;
  }
  if (first_name) {
    return first_name;
  }
  if (last_name) {
    return last_name;
  }
  return "";
}
