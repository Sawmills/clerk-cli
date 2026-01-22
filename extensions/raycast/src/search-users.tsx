import { List, ActionPanel, Action, showToast, Toast, open, LocalStorage } from "@raycast/api";
import { useState, useEffect } from "react";
import { getClerkClient, User, getUserDisplayName, getUserPrimaryEmail } from "./api/clerk";
import { useInstance } from "./hooks/useInstance";
import React from "react";
import GenerateJWT from "./generate-jwt";

export default function SearchUsers() {
  const { instance, isLoading: instanceLoading } = useInstance();
  const [users, setUsers] = useState<User[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [searchText, setSearchText] = useState("");
  const [recentUserIds, setRecentUserIds] = useState<string[]>([]);

  useEffect(() => {
    loadRecents();
  }, []);

  useEffect(() => {
    if (instanceLoading) return;
    loadUsers("");
  }, [instanceLoading]);

  useEffect(() => {
    const timer = setTimeout(() => {
      loadUsers(searchText);
    }, 300);
    return () => clearTimeout(timer);
  }, [searchText]);

  async function loadRecents() {
    const stored = await LocalStorage.getItem<string>("recent-search-users");
    if (stored) setRecentUserIds(JSON.parse(stored));
  }

  async function saveRecentUser(uid: string) {
    const updated = [uid, ...recentUserIds.filter((id) => id !== uid)].slice(0, 5);
    setRecentUserIds(updated);
    await LocalStorage.setItem("recent-search-users", JSON.stringify(updated));
  }

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

  async function impersonateUser(user: User) {
    try {
      await saveRecentUser(user.id);
      showToast({ style: Toast.Style.Animated, title: "Generating sign-in link..." });

      const client = getClerkClient();
      const token = await client.createSignInToken(user.id);

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

  const sortedUsers = [...users].sort((a, b) => {
    const aIndex = recentUserIds.indexOf(a.id);
    const bIndex = recentUserIds.indexOf(b.id);
    if (aIndex === -1 && bIndex === -1) return 0;
    if (aIndex === -1) return 1;
    if (bIndex === -1) return -1;
    return aIndex - bIndex;
  });

  return (
    <List
      isLoading={isLoading}
      onSearchTextChange={setSearchText}
      searchBarPlaceholder="Search users by name or email..."
      navigationTitle={instance ? `Users · ${instance.name}` : "Search Users"}
      throttle
    >
      {sortedUsers.length === 0 && !isLoading && (
        <List.EmptyView title="No users found" description={searchText ? "Try a different search query" : "No users in this instance"} />
      )}
      {sortedUsers.map((user) => {
        const displayName = getUserDisplayName(user);
        const email = getUserPrimaryEmail(user);
        return (
          <List.Item
            key={user.id}
            title={displayName || email || "Unknown User"}
            subtitle={email || ""}
            accessories={recentUserIds.includes(user.id) ? [{ text: "Recent" }, { text: user.id }] : [{ text: user.id }]}
            actions={
              <ActionPanel>
                <ActionPanel.Section>
                  <Action
                    title="Impersonate User"
                    icon={{ source: "person-circle" }}
                    onAction={() => impersonateUser(user)}
                  />
                  <Action.Push title="Generate JWT" target={<GenerateJWT userId={user.id} />} />
                </ActionPanel.Section>
                <ActionPanel.Section>
                  <Action.CopyToClipboard content={user.id} title="Copy User ID" />
                </ActionPanel.Section>
              </ActionPanel>
            }
          />
        );
      })}
    </List>
  );
}
