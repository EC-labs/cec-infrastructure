import { createContext } from "react"

type AuthContextType = {
  user: {
    email: string,
    role : string,
  };
  setUser: React.Dispatch<React.SetStateAction<{email: string, role: string}>>;
};

export const AuthContext = createContext<AuthContextType>({
    user: { email: "", role: "student"},
    setUser: () => {},
});
