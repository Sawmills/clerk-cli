import { List, ActionPanel, Action, showToast, Toast, Clipboard } from "@raycast/api";
import { useState, useEffect } from "react";
import { getClerkClient, User, JwtTemplate, getUserDisplayName, getUserPrimaryEmail } from "./api/clerk";
import React from "react";

export default function GenerateJWT({ userId }: { userId?: string }) {
  const [step, setStep] = useState<"user" | "template">(userId ? "template" : "user");
  const [selectedUserId, setSelectedUserId] = useState<string | undefined>(userId);
  const [users, setUsers] = useState<User[]>([]);
  const [templates, setTemplates] = useState<JwtTemplate[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [searchText, setSearchText] = useState("");

  useEffect(() => {
    if (step === "template") {
      loadTemplates();
    }
  }, [step]);

  useEffect(() => {
    if (step !== "user") return;

    const timer = setTimeout(() => {
      if (searchText) {
        loadUsers(searchText);
      } else {
        setUsers([]);
      }
    }, 500);
    return () => clearTimeout(timer);
  }, [searchText, step]);

  async function loadTemplates() {
    setIsLoading(true);
    try {
      const client = getClerkClient();
      const data = await client.listJwtTemplates();
      setTemplates(data);
    } catch (error) {
      showToast({
        style: Toast.Style.Failure,
        title: "Failed to load JWT templates",
        message: error instanceof Error ? error.message : "Unknown error",
      });
    } finally {
      setIsLoading(false);
    }
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

  async function generateJWT(uid: string, templateName: string) {
    try {
      showToast({ style: Toast.Style.Animated, title: "Generating JWT..." });

      const client = getClerkClient();
      const result = await client.createUserJwt(uid, templateName);

      await Clipboard.copy(result.token);

      showToast({
        style: Toast.Style.Success,
        title: "JWT copied to clipboard",
        message: `Token for template "${templateName}" copied`,
      });
    } catch (error) {
      showToast({
        style: Toast.Style.Failure,
        title: "Failed to generate JWT",
        message: error instanceof Error ? error.message : "Unknown error",
      });
    }
  }

  if (step === "user") {
    return (
      <List
        isLoading={isLoading}
        onSearchTextChange={setSearchText}
        searchBarPlaceholder="Search user for JWT generation..."
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
                  <Action
                    title="Select User"
                    onAction={() => {
                      setSelectedUserId(user.id);
                      setStep("template");
                    }}
                  />
                </ActionPanel>
              }
            />
          );
        })}
      </List>
    );
  }

  if (step === "template" && selectedUserId) {
    return (
      <List isLoading={isLoading} searchBarPlaceholder="Select JWT template...">
        {templates.length === 0 && !isLoading && (
          <List.EmptyView
            title="No JWT templates found"
            description="Create JWT templates in your Clerk Dashboard first. Go to: Dashboard → JWT Templates"
            actions={
              <ActionPanel>
                <Action
                  title="Go Back to User Selection"
                  onAction={() => {
                    setStep("user");
                    setSelectedUserId(undefined);
                  }}
                />
              </ActionPanel>
            }
          />
        )}
        {templates.length === 0 && isLoading && (
          <List.EmptyView title="Loading templates..." description="Please wait..." />
        )}
        {templates.map((template) => (
          <List.Item
            key={template.id}
            title={template.name}
            subtitle={`Lifetime: ${template.lifetime}s`}
            actions={
              <ActionPanel>
                <Action title="Generate JWT" onAction={() => generateJWT(selectedUserId, template.name)} />
                <Action
                  title="Go Back to User Selection"
                  shortcut={{ modifiers: ["cmd"], key: "b" }}
                  onAction={() => {
                    setStep("user");
                    setSelectedUserId(undefined);
                  }}
                />
              </ActionPanel>
            }
          />
        ))}
      </List>
    );
  }

  return (
    <List>
      <List.EmptyView title="Loading..." description="Initializing JWT generation" />
    </List>
  );
}
