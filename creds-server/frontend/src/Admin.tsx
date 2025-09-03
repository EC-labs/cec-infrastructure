import {useEffect, useState} from 'react';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import TextField from '@mui/material/TextField';
import SendIcon from '@mui/icons-material/Send';

import './admin.css';

type User = {
    email: string, 
    role: string,
    client: number,
    group: number,
}

type UserListProps = {
    users: User[]
}

export default function UserList(props: UserListProps) {
    const { users } = props;
    const [toEmail, setToEmail] = useState<undefined | string>(undefined);

    useEffect(() => {
        if (!toEmail) 
            return;
        fetch(`/api/user/${toEmail}/send_email`, { method: "POST" })
            .then((res) => {
                if (!res.ok) {
                    throw new Error(`Request status not OK: ${res.status}`);
                }
                setToEmail(undefined);
                console.log(`emailed ${toEmail}`);
            })
            .catch((e) => {
                console.log(e)
            })
    }, [toEmail]);

    function sendEmail(email: string) {
        return () => {
            setToEmail(email);
        };
    }
    return (
        <TableContainer className="creds-table-container">
            <Table sx={{ minWidth: "500px" }} aria-label="simple table">
                <TableHead>
                    <TableRow>
                        <TableCell align="center">Email</TableCell>
                        <TableCell align="center">Role</TableCell>
                        <TableCell align="center">Client</TableCell>
                        <TableCell align="center">Group</TableCell>
                        <TableCell/>
                    </TableRow>
                </TableHead>
                <TableBody>
                    {users.map((user) => (
                        <TableRow
                        key={user.email}
                        >
                            <TableCell align="left">{user.email}</TableCell>
                            <TableCell align="left">{user.role}</TableCell>
                            <TableCell align="right">{user.client}</TableCell>
                            <TableCell align="right">{user.group}</TableCell>
                            <TableCell align="center">
                                <button className="send-button" onClick={sendEmail(user.email)}><SendIcon/></button>
                            </TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
        </TableContainer>
    );
}

type AddUserProps = {
    setUsers: React.Dispatch<React.SetStateAction<User[]>>
};

function AddUser(props: AddUserProps) {
    const [email, setEmail] = useState("");
    const [add, setAdd] = useState(false)
    const { setUsers } = props;

    useEffect(() => {
        if (!add) return;
        fetch("/api/users", {
            method: "POST", 
            headers: {
              'Accept': 'application/json',
              'Content-Type': 'application/json'
            },
            body: JSON.stringify({ email })
        })
            .then((res) => {
                if (!res.ok) {
                    throw new Error(`Request status not OK: ${res.status}`);
                }
                return res.json();
            })
            .then((body) => {
                setUsers((users) => [...users, body])
            })
            .catch((e) => {
                console.log(e)
            })

        setAdd(false);
        setEmail("");
    }, [add])

    function handleInputChange(e: React.ChangeEvent<HTMLInputElement>) {
        setEmail(e.target.value);
    }

    function handleKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
        if (e.key === "Enter") {
            setAdd(true);
        }
    }

    return (
        <div className="add-user-container"> 
            <TextField 
                id="outlined-basic" 
                label="email" 
                variant="outlined" 
                value={email}
                onChange={handleInputChange}
                onKeyDown={handleKeyDown}
            />
            <button className='add-button' onClick={() => setAdd(true)}>Add</button>
        </div> 
    );

}

export function Admin() {
    const [users, setUsers] = useState<User[]>([]);

    useEffect(() => {
        fetch("/api/users")
            .then((res) => {
                if (!res.ok) {
                    throw new Error(`Request status not OK: ${res.status}`);
                }
                return res.json();
            })
            .then((body) => {
                setUsers(body);
            })
            .catch((e) => {
                console.log(e)
            })
    }, [])

    return (
        <div className="admin-page">
            <AddUser setUsers={setUsers}/>    
            <UserList users={users}/>
        </div>
    );
}
