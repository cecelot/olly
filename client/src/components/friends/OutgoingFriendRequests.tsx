import { TOAST_ERROR_OPTIONS, TOAST_SUCCESS_OPTIONS } from "@/lib";
import call from "@/lib/call";
import useOutgoingFriendRequests from "@/lib/hooks/useOutgoingFriendRequests";
import { Button } from "@headlessui/react";
import toast from "react-hot-toast";

export default function OutgoingFriendRequests() {
  const { isLoading, outgoingFriendRequests } = useOutgoingFriendRequests();

  const cancelFriendRequest = (recipient: string) => {
    return async (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
      e.preventDefault();
      (async () => {
        const res = await call(`/@me/friends/outgoing/${recipient}`, "DELETE");
        if (res.status === 200) {
          toast.success(
            `Cancelled friend request to ${recipient}.`,
            TOAST_SUCCESS_OPTIONS
          );
        } else {
          toast.error(`Failed to cancel friend request.`, TOAST_ERROR_OPTIONS);
        }
      })();
    };
  };

  if (isLoading) return <></>;

  return (outgoingFriendRequests?.length || 0) > 0 ? (
    outgoingFriendRequests?.map((request) => (
      <li
        className="flex flex-row space-x-1 justify-center"
        key={`${request.recipient}-outgoing`}
      >
        <p className="text-text">{request.recipient}</p>
        <Button
          className="text-mauve hover:text-pink transition-all"
          onClick={cancelFriendRequest(request.recipient)}
        >
          {"["}Cancel{"]"}
        </Button>
      </li>
    ))
  ) : (
    <></>
  );
}
