import useSWR, { SWRResponse } from "swr";
import { BASE_API_URL } from "@/lib";

interface MeRoute {
  message: {
    username: string;
    id: string;
  };
  code: number;
}

export default function useUser() {
  const { data, isLoading, mutate }: SWRResponse<MeRoute> = useSWR(
    `${BASE_API_URL}/@me`,
    async (url) => {
      const res = await fetch(url, {
        credentials: "include",
        mode: "cors",
      });
      const data = await res.json();
      return data;
    }
  );

  return {
    authenticated: data?.code === 200,
    user: data?.message || null,
    isLoading,
    mutate,
  };
}
