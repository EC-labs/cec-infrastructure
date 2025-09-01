import { createContext } from "react"

type AuthContextType = {
  user: string;
  setUser: React.Dispatch<React.SetStateAction<string>>;
};

export const AuthContext = createContext<AuthContextType>({
    user: "",
    setUser: () => {},
});
