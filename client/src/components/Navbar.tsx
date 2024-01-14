import { A } from "@solidjs/router";

export default function Navbar() {
  return (
    <nav class="flex flex-row justify-center p-5 bg-gray-900">
      <div class="space-x-3">
        <A
          href="/about"
          class="text-green-400 hover:text-green-500 transition-all"
        >
          {"["}About{"]"}
        </A>
        <A
          href="/stats"
          class="text-green-400 hover:text-green-500 transition-all"
        >
          {"["}Stats{"]"}
        </A>
        <A
          href="/friends"
          class="text-green-400 hover:text-green-500 transition-all"
        >
          {"["}Friends{"]"}
        </A>
        <A
          href="/login"
          class="text-green-400 hover:text-green-500 transition-all"
        >
          {"["}Login{"]"}
        </A>
      </div>
    </nav>
  );
}
