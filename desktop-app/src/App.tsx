import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [message, setMessage] = useState("");

  async function connectBackend() {
    const response = await invoke<string>("greet", {
      name: "Desktop UI",
    });

    setMessage(response);
  }

  return (
    <div className="h-screen bg-[#0f1115] text-white flex">
      {/* Sidebar */}
      <div className="w-64 border-r border-white/10 p-4">
        <h1 className="text-2xl font-bold">OpenBridge</h1>

        <div className="mt-8 space-y-2">
          <button className="w-full rounded-xl bg-white/5 p-3 text-left">
            Devices
          </button>

          <button className="w-full rounded-xl bg-white/5 p-3 text-left">
            Transfers
          </button>

          <button className="w-full rounded-xl bg-white/5 p-3 text-left">
            Settings
          </button>
        </div>
      </div>

      {/* Main */}
      <div className="flex-1 p-8">
        <div className="rounded-3xl border border-white/10 bg-white/5 p-8 h-full">
          <h2 className="text-3xl font-semibold">
            Rust Backend Test
          </h2>

          <p className="mt-3 text-white/60">
            {message || "Backend not connected"}
          </p>

          <button
            onClick={connectBackend}
            className="mt-8 rounded-2xl bg-white text-black px-6 py-3 font-medium"
          >
            Connect Backend
          </button>
        </div>
      </div>
    </div>
  );
}

export default App;