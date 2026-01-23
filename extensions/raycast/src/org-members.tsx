import { List, ActionPanel, Action, showToast, Toast, open, clearSearchBar, LocalStorage, Icon } from "@raycast/api";
import { useState, useEffect } from "react";
import { getClerkClient, Organization, Membership, getMemberDisplayName } from "./api/clerk";
import { useInstance } from "./hooks/useInstance";
import React from "react";
import GenerateJWT from "./generate-jwt";

type Step = "select-org" | "view-members";

export default function OrgMembers({ orgId }: { orgId?: string }) {
  const { instance, isLoading: instanceLoading } = useInstance();
  const [step, setStep] = useState<Step>(orgId ? "view-members" : "select-org");
  const [organizations, setOrganizations] = useState<Organization[]>([]);
  const [members, setMembers] = useState<Membership[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [searchText, setSearchText] = useState("");
  const [recentOrgIds, setRecentOrgIds] = useState<string[]>([]);
  const [recentMemberIds, setRecentMemberIds] = useState<string[]>([]);
  const [selectedOrg, setSelectedOrg] = useState<{ id: string; name: string } | null>(
    orgId ? { id: orgId, name: "Organization" } : null
  );

  const recentMembersKey = selectedOrg?.id ? `recent-org-members-${selectedOrg.id}` : null;

  useEffect(() => {
    loadRecentOrgs();
  }, []);

  useEffect(() => {
    if (instanceLoading) return;
    
    if (step === "select-org") {
      loadOrganizations();
    } else if (step === "view-members" && selectedOrg) {
      loadMembers(selectedOrg.id);
      loadRecentMembers();
    }
  }, [step, selectedOrg?.id, instanceLoading]);

  async function loadRecentOrgs() {
    const stored = await LocalStorage.getItem<string>("recent-orgs");
    if (stored) {
      setRecentOrgIds(JSON.parse(stored));
    }
  }

  async function loadRecentMembers() {
    if (!recentMembersKey) return;
    const stored = await LocalStorage.getItem<string>(recentMembersKey);
    if (stored) {
      setRecentMemberIds(JSON.parse(stored));
    } else {
      setRecentMemberIds([]);
    }
  }

  async function saveRecentOrg(orgId: string) {
    const updated = [orgId, ...recentOrgIds.filter((id) => id !== orgId)].slice(0, 5);
    setRecentOrgIds(updated);
    await LocalStorage.setItem("recent-orgs", JSON.stringify(updated));
  }

  async function saveRecentMember(userId: string) {
    if (!recentMembersKey) return;
    const updated = [userId, ...recentMemberIds.filter((id) => id !== userId)].slice(0, 5);
    setRecentMemberIds(updated);
    await LocalStorage.setItem(recentMembersKey, JSON.stringify(updated));
  }

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

  async function loadMembers(orgIdToLoad: string) {
    setIsLoading(true);
    try {
      const client = getClerkClient();
      const data = await client.listOrgMembers(orgIdToLoad);
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
      const token = await client.createSignInToken(userId, selectedOrg?.id);

      await open(token.url);

      await showToast({
        style: Toast.Style.Success,
        title: "Browser Opened!",
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

  async function selectOrganization(org: Organization) {
    await saveRecentOrg(org.id);
    setSearchText("");
    await clearSearchBar({ forceScrollToTop: true });
    setSelectedOrg({ id: org.id, name: org.name });
    setStep("view-members");
  }

  if (step === "select-org") {
    const sortedOrganizations = [...organizations].sort((a, b) => {
      const aIndex = recentOrgIds.indexOf(a.id);
      const bIndex = recentOrgIds.indexOf(b.id);
      if (aIndex === -1 && bIndex === -1) return 0;
      if (aIndex === -1) return 1;
      if (bIndex === -1) return -1;
      return aIndex - bIndex;
    });

    const filteredOrganizations = sortedOrganizations.filter((org) => {
      const query = searchText.toLowerCase();
      return (
        org.name.toLowerCase().includes(query) ||
        org.slug?.toLowerCase().includes(query) ||
        org.id.toLowerCase().includes(query)
      );
    });

    return (
      <List
        isLoading={isLoading}
        searchText={searchText}
        onSearchTextChange={setSearchText}
        searchBarPlaceholder="Search organization..."
        navigationTitle={instance ? `Orgs · ${instance.name}` : "Organizations"}
      >
        {filteredOrganizations.map((org) => (
          <List.Item
            key={org.id}
            title={org.name}
            subtitle={org.slug || ""}
            accessories={recentOrgIds.includes(org.id) ? [{ text: "Recent" }] : []}
            actions={
              <ActionPanel>
                <Action title="View Members" onAction={() => selectOrganization(org)} />
                <Action.CopyToClipboard content={org.id} title="Copy Organization ID" />
              </ActionPanel>
            }
          />
        ))}
      </List>
    );
  }

  const sortedMembers = [...members].sort((a, b) => {
    const aIndex = recentMemberIds.indexOf(a.public_user_data.user_id);
    const bIndex = recentMemberIds.indexOf(b.public_user_data.user_id);
    if (aIndex === -1 && bIndex === -1) return 0;
    if (aIndex === -1) return 1;
    if (bIndex === -1) return -1;
    return aIndex - bIndex;
  });

  return (
    <List
      isLoading={isLoading}
      searchBarPlaceholder={`Search members in ${selectedOrg?.name || "organization"}...`}
      navigationTitle={instance ? `${selectedOrg?.name} · ${instance.name}` : selectedOrg?.name}
    >
      {sortedMembers.length === 0 && !isLoading && (
        <List.EmptyView title="No members found" description="This organization has no members" />
      )}
      {sortedMembers.map((member) => {
        const displayName = getMemberDisplayName(member);
        const identifier = member.public_user_data.identifier;
        return (
          <List.Item
            key={member.id}
            title={displayName || identifier || "Unknown Member"}
            subtitle={identifier || ""}
            keywords={[displayName, identifier || "", member.role].filter(Boolean)}
            accessories={
              recentMemberIds.includes(member.public_user_data.user_id)
                ? [{ text: "Recent" }, { text: member.role }]
                : [{ text: member.role }]
            }
            actions={
              <ActionPanel>
                <ActionPanel.Section>
                  <Action
                    title="Impersonate User"
                    icon={{ source: "person-circle" }}
                    onAction={async () => {
                      await saveRecentMember(member.public_user_data.user_id);
                      await impersonateUser(member.public_user_data.user_id);
                    }}
                  />
                  <Action.Push
                    title="Generate JWT"
                    target={
                      <GenerateJWT
                        userId={member.public_user_data.user_id}
                        orgId={selectedOrg?.id}
                        userEmail={identifier || undefined}
                      />
                    }
                    onPush={async () => {
                      await saveRecentMember(member.public_user_data.user_id);
                    }}
                  />
                </ActionPanel.Section>
                <ActionPanel.Section>
                  <Action.CopyToClipboard content={member.public_user_data.user_id} title="Copy User ID" />
                </ActionPanel.Section>
              </ActionPanel>
            }
          />
        );
      })}
    </List>
  );
}
