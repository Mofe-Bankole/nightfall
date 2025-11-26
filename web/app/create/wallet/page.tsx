"use client";

import { Button } from "@/components/ui/button";

import { Input } from "@/components/ui/input";
import React from "react";

export default function Home() {
  const [password, setPassword] = React.useState("");
  const [confirmPassword, setConfirmPassword] = React.useState("");

  const isEightChars = password.length >= 8;
  const passwordsMatch = password && password === confirmPassword;


  const helperText = React.useMemo(() => {
    if (!isEightChars) {
      return "Password must be 8 characters long.";
    }
    if (!passwordsMatch) {
      return "Passwords do not match.";
    }
 
    return "";
  }, [isEightChars, passwordsMatch]);

  return (
    <div className="min-h-screen bg-black text-white flex items-center justify-center">
      <div className="w-full max-w-md rounded-3xl border border-white/10 bg-black p-8 shadow-2xl">
        <div className="mb-6 text-center">
          <p className="text-xs uppercase tracking-[0.3em] text-white/70">
            On_board_ing
          </p>
          <h1 className="mt-2 text-3xl font-semibold text-white">Create a password</h1>
          <p className="mt-3 text-sm text-white/70">
            You will use this to unlock your wallet.
          </p>
        </div>

        <form
          className="space-y-5"
          onSubmit={(e) => {
            e.preventDefault();
          }}
        >
          <div className="space-y-2">
            <div className="space-y-2.5 flex flex-col">
            <label className="text-sm text-white/80" htmlFor="password">
              Password
            </label>
            <Input
              id="password"
              type="password"
              value={password}
              onChange={(event) => setPassword(event.target.value)}
              className="bg-black text-white placeholder:text-white/40 border-white/30 focus-visible:ring-white"
              placeholder="Enter password"
            />
          </div>
            </div>
           

          <div className="space-y-2">
            <div className="flex flex-col space-y-2.5">
            <label className="text-sm text-white/80" htmlFor="confirmPassword">
              Confirm password
            </label>
            <Input
              id="confirmPassword"
              type="password"
              value={confirmPassword}
              onChange={(event) => setConfirmPassword(event.target.value)}
              className="bg-black text-white placeholder:text-white/40 border-white/30 focus-visible:ring-white"
              placeholder="Re-enter password"
            />
          </div>
          </div>
          {helperText && (
            <p className="text-xs text-white/70">{helperText}</p>
          )}

          <label className="flex items-center gap-3 text-sm text-white/80">

          </label>

          <Button
            type="submit"
            // disabled={!canContinue}
            className="w-full cursor-pointer rounded-2xl border border-white/20 bg-black py-6 text-base font-semibold text-white transition hover:bg-white hover:text-black disabled:cursor-not-allowed disabled:border-white/10 disabled:text-white/50"
          >
            Continue
          </Button>

        <p> </p>
        </form>
      </div>
    </div>
  );
}
