import { List, ActionPanel, Action, showToast, Toast, open } from "@raycast/api";
import { useState, useEffect } from "react";
import { getClerkClient, Organization, Membership, getMemberDisplayName } from "./api/clerk";
import React from "react";
import GenerateJWT from "./generate-jwt";

export default function OrgMembers({ orgId }: { orgId?: string }) {
  const [step, setStep] = useState<"org" | "members">(orgId ? "members" : "org");
  const [selectedOrgId, setSelectedOrgId] = useState<string | undefined>(orgId);
  const [organizations, setOrganizations] = useState<Organization[]>([]);
  const [members, setMembers] = useState<Membership[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [searchText, setSearchText] = useState("");

  useEffect(() => {
    if (step === "org") {
      loadOrganizations();
    }
  }, [step]);

  useEffect(() => {
    if (step === "members" && selectedOrgId) {
      loadMembers(selectedOrgId);
    }
  }, [step, selectedOrgId]);

  async function loadOrganizations() {
    setIsLoading(true);
    try {
      const client = getClerkClient();
      const data = await client.listOrganizations(100);
      setOrganizations(data);
    } catch (error) {
      showToast({
        style: Toast.Style.Failure,
        title: "Failed to load organizations",
        message: error instanceof Error ? error.message : "Unknown error",
      });
    } finally {
      setIsLoading(false);
    }
  }

  async function loadMembers(oid: string) {
    setIsLoading(true);
    try {
      const client = getClerkClient();
      const data = await client.listOrgMembers(oid);
      setMembers(data);
    } catch (error) {
      showToast({
        style: Toast.Style.Failure,
        title: "Failed to load members",
        message: error instanceof Error ? error.message : "Unknown error",
      });
    } finally {
      setIsLoading(false);
    }
  }

  async function impersonateUser(userId: string) {
    try {
      showToast({ style: Toast.Style.Animated, title: "Generating sign-in link..." });

      const client = getClerkClient();
      const token = await client.createSignInToken(userId);

      await open(token.url);

      await showToast({
        style: Toast.Style.Success,
        title: "✅ Browser Opened!",
        message: "User impersonation link opened in your browser",
      });
    } catch (error) {
      showToast({
        style: Toast.Style.Failure,
        title: "Failed to impersonate user",
        message: error instanceof Error ? error.message : "Unknown error",
      });
    }
  }

  const filteredOrganizations = organizations.filter((org) => {
    const query = searchText.toLowerCase();
    return (
      org.name.toLowerCase().includes(query) ||
      org.slug?.toLowerCase().includes(query) ||
      org.id.toLowerCase().includes(query)
    );
  });

  if (step === "org") {
    return (
      <List
        isLoading={isLoading}
        onSearchTextChange={setSearchText}
        searchBarPlaceholder="Search organization..."
      >
        {filteredOrganizations.map((org) => (
          <List.Item
            key={org.id}
            title={org.name}
            subtitle={org.slug || ""}
            actions={
              <ActionPanel>
                <Action
                  title="View Members"
                  onAction={() => {
                    setSelectedOrgId(org.id);
                    setStep("members");
                  }}
                />
              </ActionPanel>
            }
          />
        ))}
      </List>
    );
  }

  if (step === "members" && selectedOrgId) {
    return (
      <List isLoading={isLoading} searchBarPlaceholder="Search members...">
        {members.length === 0 && !isLoading && (
          <List.EmptyView title="No members found" description="This organization has no members" />
        )}
        {members.map((member) => {
          const displayName = getMemberDisplayName(member);
          const identifier = member.public_user_data.identifier;
          return (
            <List.Item
              key={member.id}
              title={displayName || identifier || "Unknown Member"}
              subtitle={identifier || ""}
              accessories={[{ text: member.role }]}
              actions={
                <ActionPanel>
                  <Action
                    title="Impersonate User"
                    onAction={() => impersonateUser(member.public_user_data.user_id)}
                  />
                  <Action.Push
                    title="Generate JWT"
                    target={<GenerateJWT userId={member.public_user_data.user_id} />}
                  />
                  <Action.CopyToClipboard content={member.public_user_data.user_id} title="Copy User ID" />
                </ActionPanel>
              }
            />
          );
        })}
      </List>
    );
  }

  return null;
}
