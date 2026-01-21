import { List, ActionPanel, Action, showToast, Toast, open } from "@raycast/api";
import { useState, useEffect } from "react";
import { getClerkClient, User, getUserDisplayName, getUserPrimaryEmail } from "./api/clerk";
import React from "react";

export default function ImpersonateUser({ userId }: { userId?: string }) {
  const [users, setUsers] = useState<User[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [searchText, setSearchText] = useState("");

  useEffect(() => {
    if (userId) {
      impersonateUser(userId);
    }
  }, [userId]);

  useEffect(() => {
    if (userId) return;

    const timer = setTimeout(() => {
      if (searchText) {
        loadUsers(searchText);
      } else {
        setUsers([]);
      }
    }, 500);
    return () => clearTimeout(timer);
  }, [searchText, userId]);

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

  return (
    <List
      isLoading={isLoading}
      onSearchTextChange={setSearchText}
      searchBarPlaceholder="Search user to impersonate..."
      throttle
    >
      {users.length === 0 && !isLoading && searchText && (
        <List.EmptyView title="No users found" description="Try a different search query" />
      )}
      {users.length === 0 && !isLoading && !searchText && (
        <List.EmptyView title="Search for a user" description="Start typing to search by name or email" />
      )}
      {users.map((user) => {
        const displayName = getUserDisplayName(user);
        const email = getUserPrimaryEmail(user);
        return (
          <List.Item
            key={user.id}
            title={displayName || email || "Unknown User"}
            subtitle={email || ""}
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
