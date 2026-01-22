import { List, ActionPanel, Action, showToast, Toast, open, LocalStorage } from "@raycast/api";
import { useState, useEffect } from "react";
import { getClerkClient, User, getUserDisplayName, getUserPrimaryEmail } from "./api/clerk";
import { useInstance } from "./hooks/useInstance";
import React from "react";

export default function ImpersonateUser({ userId }: { userId?: string }) {
  const { instance, isLoading: instanceLoading } = useInstance();
  const [users, setUsers] = useState<User[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [searchText, setSearchText] = useState("");
  const [recentUserIds, setRecentUserIds] = useState<string[]>([]);

  useEffect(() => {
    if (!userId) {
      loadRecents();
    }
  }, [userId]);

  useEffect(() => {
    if (instanceLoading) return;
    if (userId) {
      impersonateUser(userId);
    } else {
      loadUsers("");
    }
  }, [userId, instanceLoading]);

  useEffect(() => {
    if (userId) return;

    const timer = setTimeout(() => {
      loadUsers(searchText);
    }, 300);
    return () => clearTimeout(timer);
  }, [searchText, userId]);

  async function loadRecents() {
    const stored = await LocalStorage.getItem<string>("recent-impersonate-users");
    if (stored) setRecentUserIds(JSON.parse(stored));
  }

  async function saveRecentUser(uid: string) {
    const updated = [uid, ...recentUserIds.filter((id) => id !== uid)].slice(0, 5);
    setRecentUserIds(updated);
    await LocalStorage.setItem("recent-impersonate-users", JSON.stringify(updated));
  }

  async function loadUsers(query: string) {
    setIsLoading(true);
    try {
      const client = getClerkClient();
      const data = await client.listUsers(query, 20);
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

  async function impersonateUser(uid: string) {
    try {
      await saveRecentUser(uid);
      showToast({ style: Toast.Style.Animated, title: "Generating sign-in link..." });

      const client = getClerkClient();
      const token = await client.createSignInToken(uid);

      await open(token.url);

      showToast({
        style: Toast.Style.Success,
        title: "Sign-in link opened",
        message: "User impersonation link opened in browser",
      });
    } catch (error) {
      showToast({
        style: Toast.Style.Failure,
        title: "Failed to impersonate user",
        message: error instanceof Error ? error.message : "Unknown error",
      });
    }
  }

  if (userId) {
    return null;
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
      searchBarPlaceholder="Search user to impersonate..."
      navigationTitle={instance ? `Impersonate · ${instance.name}` : "Impersonate User"}
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
            accessories={recentUserIds.includes(user.id) ? [{ text: "Recent" }] : []}
            actions={
              <ActionPanel>
                <Action title="Impersonate User" onAction={() => impersonateUser(user.id)} />
              </ActionPanel>
            }
          />
        );
      })}
    </List>
  );
}
