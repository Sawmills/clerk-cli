import { getPreferenceValues } from "@raycast/api";

const BASE_URL = "https://api.clerk.com/v1";

// Preferences interface
interface Preferences {
  clerkApiKey: string;
  clerkFrontendApi?: string;
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

  constructor(apiKey: string) {
    this.apiKey = apiKey;
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

  async createUserJwt(userId: string, templateName: string): Promise<{ token: string }> {
    // First, get the user's active sessions
    const sessions = await this.request<Session[]>(`/sessions?user_id=${userId}&status=active`);

    if (sessions.length === 0) {
      throw new Error("No active sessions found for user");
    }

    // Use the first active session to create the JWT
    const sessionId = sessions[0].id;
    const response = await this.request<SessionTokenResponse>(`/sessions/${sessionId}/tokens/${templateName}`, {
      method: "POST",
    });

    return { token: response.jwt };
  }
}

// Helper function to get a configured client instance
export function getClerkClient(): ClerkAPI {
  const preferences = getPreferenceValues<Preferences>();

  if (!preferences.clerkApiKey) {
    throw new Error("Clerk API key is required. Please configure it in extension preferences.");
  }

  return new ClerkAPI(preferences.clerkApiKey);
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
