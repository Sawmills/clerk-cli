import { List, ActionPanel, Action, showToast, Toast, Clipboard, clearSearchBar, LocalStorage } from "@raycast/api";
import { useState, useEffect } from "react";
import { getClerkClient, User, JwtTemplate, getUserDisplayName, getUserPrimaryEmail } from "./api/clerk";
import { useInstance } from "./hooks/useInstance";
import React from "react";

export default function GenerateJWT({ userId, orgId, userEmail }: { userId?: string; orgId?: string; userEmail?: string }) {
  const { instance, isLoading: instanceLoading } = useInstance();
  const [step, setStep] = useState<"user" | "template">(userId ? "template" : "user");
  const [selectedUserId, setSelectedUserId] = useState<string | undefined>(userId);
  const [selectedUserEmail, setSelectedUserEmail] = useState<string | undefined>(userEmail);
  const [users, setUsers] = useState<User[]>([]);
  const [templates, setTemplates] = useState<JwtTemplate[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [searchText, setSearchText] = useState("");
  const [recentUserIds, setRecentUserIds] = useState<string[]>([]);
  const [recentTemplateIds, setRecentTemplateIds] = useState<string[]>([]);

  const recentUsersKey = orgId ? `recent-jwt-users-${orgId}` : "recent-jwt-users";

  useEffect(() => {
    loadRecents();
  }, []);

  useEffect(() => {
    if (instanceLoading) return;
    if (step === "template") {
      loadTemplates();
    }
  }, [step, instanceLoading]);

  useEffect(() => {
    if (instanceLoading) return;
    if (step === "user") {
      loadUsers("");
    }
  }, [step, instanceLoading]);

  useEffect(() => {
    if (step !== "user") return;

    const timer = setTimeout(() => {
      loadUsers(searchText);
    }, 300);
    return () => clearTimeout(timer);
  }, [searchText]);

  async function loadRecents() {
    const storedUsers = await LocalStorage.getItem<string>(recentUsersKey);
    const storedTemplates = await LocalStorage.getItem<string>("recent-jwt-templates");
    if (storedUsers) setRecentUserIds(JSON.parse(storedUsers));
    if (storedTemplates) setRecentTemplateIds(JSON.parse(storedTemplates));
  }

  async function saveRecentUser(uid: string) {
    const updated = [uid, ...recentUserIds.filter((id) => id !== uid)].slice(0, 5);
    setRecentUserIds(updated);
    await LocalStorage.setItem(recentUsersKey, JSON.stringify(updated));
  }

  async function saveRecentTemplate(templateId: string) {
    const updated = [templateId, ...recentTemplateIds.filter((id) => id !== templateId)].slice(0, 5);
    setRecentTemplateIds(updated);
    await LocalStorage.setItem("recent-jwt-templates", JSON.stringify(updated));
  }

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

  async function generateJWT(uid: string, template: JwtTemplate, email?: string) {
    try {
      showToast({ style: Toast.Style.Animated, title: "Generating JWT..." });
      await saveRecentTemplate(template.id);

      const client = getClerkClient();
      const result = await client.createUserJwt(uid, template.name, orgId, email ?? selectedUserEmail);

      await Clipboard.copy(result.token);

      await showToast({
        style: Toast.Style.Success,
        title: "JWT Copied!",
        message: `Token for "${template.name}" is in your clipboard.`,
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
        searchText={searchText}
        onSearchTextChange={setSearchText}
        searchBarPlaceholder="Search user for JWT generation..."
        navigationTitle={instance ? `JWT · ${instance.name}` : "Generate JWT"}
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
                  <Action
                    title="Select User"
                    onAction={async () => {
                      await saveRecentUser(user.id);
                      setSearchText("");
                      await clearSearchBar({ forceScrollToTop: true });
                      setSelectedUserId(user.id);
                      setSelectedUserEmail(email || undefined);
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
    const sortedTemplates = [...templates].sort((a, b) => {
      const aIndex = recentTemplateIds.indexOf(a.id);
      const bIndex = recentTemplateIds.indexOf(b.id);
      if (aIndex === -1 && bIndex === -1) return 0;
      if (aIndex === -1) return 1;
      if (bIndex === -1) return -1;
      return aIndex - bIndex;
    });

    return (
      <List isLoading={isLoading} searchBarPlaceholder="Select JWT template..." navigationTitle={instance ? `Templates · ${instance.name}` : "Select Template"}>
        {sortedTemplates.length === 0 && !isLoading && (
          <List.EmptyView
            title="No JWT templates found"
            description="Create JWT templates in your Clerk Dashboard first."
            actions={
              <ActionPanel>
                <Action
                  title="Go Back to User Selection"
                  onAction={() => {
                    setStep("user");
                    setSelectedUserId(undefined);
                    setSelectedUserEmail(undefined);
                  }}
                />
              </ActionPanel>
            }
          />
        )}
        {sortedTemplates.length === 0 && isLoading && (
          <List.EmptyView title="Loading templates..." description="Please wait..." />
        )}
        {sortedTemplates.map((template) => (
          <List.Item
            key={template.id}
            title={template.name}
            subtitle={`Lifetime: ${template.lifetime}s`}
            accessories={recentTemplateIds.includes(template.id) ? [{ text: "Recent" }] : []}
            actions={
              <ActionPanel>
                <Action
                  title="Generate JWT"
                  onAction={() => generateJWT(selectedUserId, template, selectedUserEmail)}
                />
                <Action
                  title="Go Back to User Selection"
                  shortcut={{ modifiers: ["cmd"], key: "b" }}
                  onAction={() => {
                    setStep("user");
                    setSelectedUserId(undefined);
                    setSelectedUserEmail(undefined);
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
