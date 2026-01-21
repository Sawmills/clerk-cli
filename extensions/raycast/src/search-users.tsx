import { List, ActionPanel, Action, showToast, Toast, open } from "@raycast/api";
import { useState, useEffect } from "react";
import { getClerkClient, User, getUserDisplayName, getUserPrimaryEmail } from "./api/clerk";
import React from "react";
import GenerateJWT from "./generate-jwt";

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
                <Action title="Impersonate User" onAction={() => impersonateUser(user.id)} />
                <Action.Push title="Generate JWT" target={<GenerateJWT userId={user.id} />} />
                <Action.CopyToClipboard content={user.id} title="Copy User ID" />
              </ActionPanel>
            }
          />
        );
      })}
    </List>
  );
}


