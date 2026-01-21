import { List, ActionPanel, Action, showToast, Toast } from "@raycast/api";
import { useState, useEffect } from "react";
import { getClerkClient, User, getUserDisplayName, getUserPrimaryEmail } from "./api/clerk";
import React from "react";

export default function SearchUsers() {
  const [users, setUsers] = useState<User[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [searchText, setSearchText] = useState("");

  useEffect(() => {
    const timer = setTimeout(() => {
      if (searchText) {
        loadUsers(searchText);
      } else {
        setUsers([]);
      }
    }, 500);
    return () => clearTimeout(timer);
  }, [searchText]);

  async function loadUsers(query: string) {
    setIsLoading(true);
    try {
      const client = getClerkClient();
      const data = await client.listUsers(query, 50);
      setUsers(data);
    } catch (error) {
      showToast({
        style: Toast.Style.Failure,
        title: "Failed to search users",
        message: error instanceof Error ? error.message : "Unknown error",
      });
    } finally {
      setIsLoading(false);
    }
  }

  return (
    <List
      isLoading={isLoading}
      onSearchTextChange={setSearchText}
      searchBarPlaceholder="Search users by name or email..."
      throttle
    >
      {users.length === 0 && !isLoading && searchText && (
        <List.EmptyView title="No users found" description="Try a different search query" />
      )}
      {users.length === 0 && !isLoading && !searchText && (
        <List.EmptyView title="Search for users" description="Start typing to search by name or email" />
      )}
      {users.map((user) => {
        const displayName = getUserDisplayName(user);
        const email = getUserPrimaryEmail(user);
        return (
          <List.Item
            key={user.id}
            title={displayName || email || "Unknown User"}
            subtitle={email || ""}
            accessories={[{ text: user.id }]}
            actions={
              <ActionPanel>
                <Action.CopyToClipboard content={user.id} title="Copy User ID" />
                <Action.Push title="Impersonate User" target={<ImpersonateUser userId={user.id} />} />
                <Action.Push title="Generate JWT" target={<GenerateJWT userId={user.id} />} />
              </ActionPanel>
            }
          />
        );
      })}
    </List>
  );
}

function ImpersonateUser({ userId }: { userId: string }) {
  return (
    <List>
      <List.Item title={`Impersonate ${userId}`} subtitle="Coming soon" />
    </List>
  );
}

function GenerateJWT({ userId }: { userId: string }) {
  return (
    <List>
      <List.Item title={`Generate JWT for ${userId}`} subtitle="Coming soon" />
    </List>
  );
}
