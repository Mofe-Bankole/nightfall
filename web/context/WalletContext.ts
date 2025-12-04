/* eslint-disable @typescript-eslint/no-unused-vars */
/* eslint-disable @typescript-eslint/no-explicit-any */
import axios from "axios";
import React, {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useState,
} from "react";

const API_BASE_URL = process.env.API_PORT;

export type Wallet = {
  seed_phrase: any;
  private_key?: string;
  public_key?: string;
  transactions: [any];
  transparent_address: string;
  shielded_address: string;
};
type User = {
  username: string;
  password: string;
  seed_phrase: string;
  wallet: Wallet;
  balance: string;
};

type CreateNewUserArgs = {
  password: string;
  username: string;
  seed_phrase: string;
};

type RetreiveExistingUser = {
  password: string;
  seed_phrase?: string;
};

type Session = {
  user: User;
  token: string;
  expires_at: Date;
};

type TransferArgs = {
  receipient_addr: string;
  sender_addr: string;
  memos: string;
};

type WalletContextValue = {
  loading?: boolean;
  signUp: (input: CreateNewUserArgs) => Promise<any>;
  signIn: (input: RetreiveExistingUser) => Promise<any>;
};

const WalletContext = createContext<WalletContextValue | null>(null);

export function WalletProvider({ children }: { children: React.ReactNode }) {
  const client = useMemo(() => {
    const instance = axios.create({
      baseURL: API_BASE_URL,
      timeout: 25000,
      headers: { "Content-Type": "application/json" },
    });

    return instance;
  }, []);

  const [session, setSession] = useState<Session | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | undefined>(undefined);

  const signUp = useCallback(
    async (input: CreateNewUserArgs) => {
      setLoading(true);
      setError(undefined);
      try {
        const { data } = await client.post("/api/v1/wallet/create", input);
        const nextSession: Session = {
          user: data?.user,
          token: data?.token,
          expires_at: data.expires_at,
        };
        setSession(nextSession);
        localStorage.setItem("user_token", nextSession.token as string);
      } catch (error: any) {
        const err =
          error?.response?.data?.message ||
          error?.message ||
          "Account creation was ehmm............unsuccessful";
        setError(err);
        throw new Error(err);
      }
    },
    [client],
  );

  const signIn = useCallback(
    async (input: RetreiveExistingUser) => {
      setLoading(true);

      try {
        const { data } = await client.post("/api/v1/auth/signin", input);
        const nextSession: Session = {
          user: data?.user,
          token: data?.token,
          expires_at: data.expires_at,
        };
        setSession(nextSession);
        localStorage.setItem("user_token", nextSession.token as string);
      } catch (error: any) {
        const err =
          error?.response?.data?.message ||
          error?.message ||
          "Seems you either didnt put in the correct credentials or you dont have a wallet created earlier";
        setError(err);
        throw new Error(err);
      } finally {
        setLoading(false);
      }
    },
    [client],
  );

  const value = useMemo<WalletContextValue>(
    () => ({
      session,
      loading,
      error,
      signUp,
      signIn,
      setSession,
    }),
    [error, loading, session, signIn, signUp],
  );
}

export function useAuth() {
  const ctx = useContext(WalletContext);

  if (!ctx) {
    throw new Error("WalletContext should be used in a provider");
  }

  return ctx;
}
