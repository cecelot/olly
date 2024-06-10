import IncomingFriendRequests from "./IncomingFriendRequests";
import OutgoingFriendRequests from "./OutgoingFriendRequests";

export default function MergedFriendRequests() {
  return (
    <section className="py-10 text-left">
      <h2 className="font-bold text-text text-center">Friend Requests</h2>
      <ul className="max-h-80 py-2 overflow-y-scroll">
        <IncomingFriendRequests />
        <OutgoingFriendRequests />
      </ul>
    </section>
  );
}
