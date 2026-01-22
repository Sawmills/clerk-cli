import { useEffect, useState } from "react";
import { getCurrentInstance, ClerkInstance, getInstanceColor } from "../api/instances";
import { loadCurrentInstance } from "../api/clerk";

export function useInstance() {
  const [instance, setInstance] = useState<ClerkInstance | undefined>();
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    async function load() {
      const inst = await loadCurrentInstance();
      setInstance(inst);
      setIsLoading(false);
    }
    load();
  }, []);

  const instanceTag = instance
    ? { text: instance.name, color: getInstanceColor(instance.name) }
    : undefined;

  return { instance, isLoading, instanceTag };
}
