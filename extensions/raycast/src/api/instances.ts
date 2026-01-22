import { LocalStorage } from "@raycast/api";

export interface ClerkInstance {
  id: string;
  name: string;
  apiKey: string;
  frontendApi?: string;
}

const INSTANCES_KEY = "clerk-instances";
const CURRENT_INSTANCE_KEY = "clerk-current-instance";

export async function getInstances(): Promise<ClerkInstance[]> {
  const stored = await LocalStorage.getItem<string>(INSTANCES_KEY);
  return stored ? JSON.parse(stored) : [];
}

export async function saveInstances(instances: ClerkInstance[]): Promise<void> {
  await LocalStorage.setItem(INSTANCES_KEY, JSON.stringify(instances));
}

export async function addInstance(instance: ClerkInstance): Promise<void> {
  const instances = await getInstances();
  const existing = instances.findIndex((i) => i.id === instance.id);
  if (existing >= 0) {
    instances[existing] = instance;
  } else {
    instances.push(instance);
  }
  await saveInstances(instances);
}

export async function removeInstance(id: string): Promise<void> {
  const instances = await getInstances();
  await saveInstances(instances.filter((i) => i.id !== id));
  
  const current = await getCurrentInstanceId();
  if (current === id) {
    await setCurrentInstanceId(undefined);
  }
}

export async function getCurrentInstanceId(): Promise<string | undefined> {
  return await LocalStorage.getItem<string>(CURRENT_INSTANCE_KEY);
}

export async function setCurrentInstanceId(id: string | undefined): Promise<void> {
  if (id) {
    await LocalStorage.setItem(CURRENT_INSTANCE_KEY, id);
  } else {
    await LocalStorage.removeItem(CURRENT_INSTANCE_KEY);
  }
}

export async function getCurrentInstance(): Promise<ClerkInstance | undefined> {
  const id = await getCurrentInstanceId();
  if (!id) return undefined;
  
  const instances = await getInstances();
  return instances.find((i) => i.id === id);
}

export function getInstanceColor(name: string): string {
  const lower = name.toLowerCase();
  if (lower.includes("prod")) return "#ef4444";
  if (lower.includes("staging") || lower.includes("stg")) return "#f59e0b";
  if (lower.includes("dev") || lower.includes("local")) return "#22c55e";
  return "#6366f1";
}
