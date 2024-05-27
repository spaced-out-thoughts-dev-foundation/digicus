import React from 'react'
import Box from '@mui/material/Box';

import ".././styles/CodeContainer.css";

function UserDefinedTypesContainer({userDefinedTypes}) {
  return (
    console.log("UserDefinedTypesContainer userDefinedTypes: ", userDefinedTypes),
    <Box className="code-container">
      <h2>User Defined Types</h2>
     { userDefinedTypes ?
        userDefinedTypes?.map((userDefinedType, index) => {
          return (
            <div key={index}>
              <h3>({index + 1}) Name: {JSON.parse(userDefinedType).name}</h3>
              <h4>Attributes:</h4>
              <ul>
                {JSON.parse(userDefinedType).attributes?.map((attribute, index) => {
                  return (
                    <li key={index}>
                      <strong>{attribute.name}</strong>: {attribute.type}
                    </li>
                  )
                })}
              </ul>
            </div>
          )
        })
      : ''}
    </Box>
  )
}
export default UserDefinedTypesContainer;