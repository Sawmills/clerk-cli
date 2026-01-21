import { List, ActionPanel, Action, showToast, Toast } from "@raycast/api";
import { useState, useEffect } from "react";
import { getClerkClient, Organization } from "./api/clerk";
import React from "react";
import OrgMembers from "./org-members";

export default function SearchOrganizations() {
  const [organizations, setOrganizations] = useState<Organization[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [searchText, setSearchText] = useState("");

  useEffect(() => {
    loadOrganizations();
  }, []);

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

  const filteredOrganizations = organizations.filter((org) => {
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
    >
      {filteredOrganizations.map((org) => (
        <List.Item
          key={org.id}
          title={org.name}
          subtitle={org.slug || ""}
          accessories={[{ text: org.id }]}
          actions={
            <ActionPanel>
              <Action.CopyToClipboard content={org.id} title="Copy Organization ID" />
              <Action.Push title="View Members" target={<OrgMembers orgId={org.id} />} />
              <Action.Push title="View Details" target={<OrgDetails organization={org} />} />
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


