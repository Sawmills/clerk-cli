import {
  List,
  ActionPanel,
  Action,
  Form,
  showToast,
  Toast,
  useNavigation,
  Icon,
  Color,
  confirmAlert,
  Alert,
} from "@raycast/api";
import { useState, useEffect } from "react";
import React from "react";
import {
  ClerkInstance,
  getInstances,
  addInstance,
  removeInstance,
  getCurrentInstanceId,
  setCurrentInstanceId,
  getInstanceColor,
} from "./api/instances";
import { getClerkClientForInstance } from "./api/clerk";

function AddInstanceForm({ onAdd }: { onAdd: () => void }) {
  const { pop } = useNavigation();
  const [isLoading, setIsLoading] = useState(false);

  async function handleSubmit(values: { name: string; apiKey: string; frontendApi?: string }) {
    if (!values.name.trim()) {
      showToast({ style: Toast.Style.Failure, title: "Name is required" });
      return;
    }
    if (!values.apiKey.trim()) {
      showToast({ style: Toast.Style.Failure, title: "API Key is required" });
      return;
    }

    setIsLoading(true);
    try {
      const client = getClerkClientForInstance({
        id: "",
        name: values.name,
        apiKey: values.apiKey,
      });
      await client.listOrganizations(1);

      const instance: ClerkInstance = {
        id: `instance-${Date.now()}`,
        name: values.name.trim(),
        apiKey: values.apiKey.trim(),
        frontendApi: values.frontendApi?.trim() || undefined,
      };

      await addInstance(instance);
      showToast({ style: Toast.Style.Success, title: "Instance added", message: instance.name });
      onAdd();
      pop();
    } catch (error) {
      showToast({
        style: Toast.Style.Failure,
        title: "Invalid API Key",
        message: "Could not connect to Clerk with this API key",
      });
    } finally {
      setIsLoading(false);
    }
  }

  return (
    <Form
      isLoading={isLoading}
      actions={
        <ActionPanel>
          <Action.SubmitForm title="Add Instance" onSubmit={handleSubmit} />
        </ActionPanel>
      }
    >
      <Form.TextField
        id="name"
        title="Name"
        placeholder="Production, Staging, Development..."
        info="A friendly name to identify this instance"
      />
      <Form.PasswordField
        id="apiKey"
        title="API Key"
        placeholder="sk_live_... or sk_test_..."
        info="Your Clerk secret key"
      />
      <Form.TextField
        id="frontendApi"
        title="Frontend API (Optional)"
        placeholder="https://clerk.yourdomain.com"
        info="For impersonation on custom domains"
      />
    </Form>
  );
}

export default function SwitchInstance() {
  const [instances, setInstances] = useState<ClerkInstance[]>([]);
  const [currentId, setCurrentId] = useState<string | undefined>();
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    loadData();
  }, []);

  async function loadData() {
    setIsLoading(true);
    const [instanceList, current] = await Promise.all([getInstances(), getCurrentInstanceId()]);
    setInstances(instanceList);
    setCurrentId(current);
    setIsLoading(false);
  }

  async function handleSelect(instance: ClerkInstance) {
    await setCurrentInstanceId(instance.id);
    setCurrentId(instance.id);
    showToast({
      style: Toast.Style.Success,
      title: "Switched to " + instance.name,
      message: "All commands will now use this instance",
    });
  }

  async function handleClearSelection() {
    await setCurrentInstanceId(undefined);
    setCurrentId(undefined);
    showToast({
      style: Toast.Style.Success,
      title: "Using default",
      message: "Commands will use the API key from preferences",
    });
  }

  async function handleDelete(instance: ClerkInstance) {
    const confirmed = await confirmAlert({
      title: "Delete Instance?",
      message: `Are you sure you want to delete "${instance.name}"?`,
      primaryAction: {
        title: "Delete",
        style: Alert.ActionStyle.Destructive,
      },
    });

    if (confirmed) {
      await removeInstance(instance.id);
      await loadData();
      showToast({ style: Toast.Style.Success, title: "Instance deleted" });
    }
  }

  return (
    <List isLoading={isLoading} searchBarPlaceholder="Search instances...">
      <List.Section title="Instances">
        {instances.map((instance) => {
          const isCurrent = instance.id === currentId;
          const color = getInstanceColor(instance.name);

          return (
            <List.Item
              key={instance.id}
              title={instance.name}
              subtitle={instance.apiKey.substring(0, 15) + "..."}
              icon={{ source: Icon.Circle, tintColor: color }}
              accessories={isCurrent ? [{ icon: Icon.Checkmark, tooltip: "Current" }] : []}
              actions={
                <ActionPanel>
                  <ActionPanel.Section>
                    {!isCurrent && (
                      <Action
                        title="Switch to This Instance"
                        icon={Icon.Switch}
                        onAction={() => handleSelect(instance)}
                      />
                    )}
                    {isCurrent && (
                      <Action
                        title="Use Default (From Preferences)"
                        icon={Icon.XmarkCircle}
                        onAction={handleClearSelection}
                      />
                    )}
                  </ActionPanel.Section>
                  <ActionPanel.Section>
                    <Action.Push
                      title="Add New Instance"
                      icon={Icon.Plus}
                      target={<AddInstanceForm onAdd={loadData} />}
                      shortcut={{ modifiers: ["cmd"], key: "n" }}
                    />
                    <Action
                      title="Delete Instance"
                      icon={Icon.Trash}
                      style={Action.Style.Destructive}
                      shortcut={{ modifiers: ["cmd"], key: "backspace" }}
                      onAction={() => handleDelete(instance)}
                    />
                  </ActionPanel.Section>
                  <ActionPanel.Section>
                    <Action.CopyToClipboard
                      title="Copy API Key"
                      content={instance.apiKey}
                      shortcut={{ modifiers: ["cmd", "shift"], key: "c" }}
                    />
                  </ActionPanel.Section>
                </ActionPanel>
              }
            />
          );
        })}
      </List.Section>

      <List.Section title="Actions">
        <List.Item
          title="Add New Instance"
          icon={{ source: Icon.Plus, tintColor: Color.Blue }}
          actions={
            <ActionPanel>
              <Action.Push title="Add Instance" target={<AddInstanceForm onAdd={loadData} />} />
            </ActionPanel>
          }
        />
        {currentId && (
          <List.Item
            title="Use Default (From Preferences)"
            subtitle="Clear instance selection"
            icon={{ source: Icon.XmarkCircle, tintColor: Color.Orange }}
            actions={
              <ActionPanel>
                <Action title="Use Default" onAction={handleClearSelection} />
              </ActionPanel>
            }
          />
        )}
      </List.Section>
    </List>
  );
}
