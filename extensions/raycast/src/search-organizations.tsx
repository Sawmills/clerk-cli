import { List, ActionPanel, Action, showToast, Toast, clearSearchBar, useNavigation, LocalStorage } from "@raycast/api";
import { useState, useEffect } from "react";
import { getClerkClient, Organization } from "./api/clerk";
import { useInstance } from "./hooks/useInstance";
import React from "react";
import OrgMembers from "./org-members";

export default function SearchOrganizations() {
  const { instance, isLoading: instanceLoading } = useInstance();
  const { push } = useNavigation();
  const [organizations, setOrganizations] = useState<Organization[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [searchText, setSearchText] = useState("");
  const [recentOrgIds, setRecentOrgIds] = useState<string[]>([]);

  useEffect(() => {
    loadRecents();
  }, []);

  useEffect(() => {
    if (instanceLoading) return;
    loadOrganizations();
  }, [instanceLoading]);

  async function loadRecents() {
    const stored = await LocalStorage.getItem<string>("recent-search-orgs");
    if (stored) setRecentOrgIds(JSON.parse(stored));
  }

  async function saveRecentOrg(orgId: string) {
    const updated = [orgId, ...recentOrgIds.filter((id) => id !== orgId)].slice(0, 5);
    setRecentOrgIds(updated);
    await LocalStorage.setItem("recent-search-orgs", JSON.stringify(updated));
  }

  async function loadOrganizations() {
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
      onSearchTextChange={setSearchText}
      searchBarPlaceholder="Search organizations..."
      navigationTitle={instance ? `Orgs · ${instance.name}` : "Search Organizations"}
    >
      {filteredOrganizations.map((org) => (
        <List.Item
          key={org.id}
          title={org.name}
          subtitle={org.slug || ""}
          accessories={recentOrgIds.includes(org.id) ? [{ text: "Recent" }, { text: org.id }] : [{ text: org.id }]}
          actions={
            <ActionPanel>
              <ActionPanel.Section>
                <Action
                  title="View Members"
                  onAction={async () => {
                    await saveRecentOrg(org.id);
                    setSearchText("");
                    await clearSearchBar({ forceScrollToTop: true });
                    push(<OrgMembers orgId={org.id} />);
                  }}
                />
                <Action.Push title="View Details" target={<OrgDetails organization={org} />} />
              </ActionPanel.Section>
              <ActionPanel.Section>
                <Action.CopyToClipboard content={org.id} title="Copy Organization ID" />
              </ActionPanel.Section>
            </ActionPanel>
          }
        />
      ))}
    </List>
  );
}

function OrgDetails({ organization }: { organization: Organization }) {
  return (
    <List>
      <List.Item title="ID" subtitle={organization.id} />
      <List.Item title="Name" subtitle={organization.name} />
      <List.Item title="Slug" subtitle={organization.slug || "N/A"} />
      <List.Item title="Members" subtitle={organization.members_count?.toString() || "N/A"} />
      <List.Item title="Created" subtitle={new Date(organization.created_at).toLocaleString()} />
    </List>
  );
}
