import { useContext } from "react"
import { AuthContext } from "./AuthContext"

export function Home() {

    const { user } = useContext(AuthContext);
    return (
        <h1>Hello {user}</h1>
    )
}
