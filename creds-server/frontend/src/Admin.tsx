import {useEffect, useState} from 'react';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import TextField from '@mui/material/TextField';

import './admin.css';

function createData(
  email: string,
  role: string,
  client: number,
  group: number | undefined
) {
  return {email, role, client, group} as User;
}

const user_data = [
  createData('d.landau@uu.nl', "admin", 0, undefined),
  createData('n.saurabh@uu.nl', "admin", 1, undefined),
  createData('student1@uu.nl', "student", 2, undefined),
];

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
  return (
    <TableContainer className="creds-table-container">
      <Table sx={{ minWidth: "500px" }} aria-label="simple table">
        <TableHead>
          <TableRow>
            <TableCell align="center">Email</TableCell>
            <TableCell align="center">Role</TableCell>
            <TableCell align="center">Client</TableCell>
            <TableCell align="center">Group</TableCell>
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
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
}

type AddUserProps = {
    setUsers: React.Dispatch<React.SetStateAction<never[]>>
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
    }, [add])

    function handleInputChange(e: React.ChangeEvent<HTMLInputElement>) {
        setEmail(e.target.value);
    }

    return (
        <div className="add-user-container"> 
            <TextField 
                id="outlined-basic" 
                label="email" 
                variant="outlined" 
                value={email}
                onChange={handleInputChange}
            />
            <button className='add-button' onClick={() => setAdd(true)}>Add</button>
        </div> 
    );

}

export function Admin() {
    const [users, setUsers] = useState([]);

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
