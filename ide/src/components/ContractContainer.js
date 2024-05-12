import React from 'react'
import Box from '@mui/material/Box';
import ReactFlow from 'reactflow';

// TODO: this only works for one function right now
function ContractContainer({functions}) {
    return (
      <div style={{
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center',
          flexDirection: 'column',
          flex: '15',
          backgroundColor: 'rgb(39 207 230)',
      }}>
        <Box
            style={{
              backgroundColor: 'white',
              color: 'black',
            }}
            height={'100%'}
            width={'80%'}
            my={4}
            display="flex"
            alignItems="center"
            gap={4}
            p={2}
            sx={{ border: '2px solid grey' }}
        >
          {
            functions ?
              <ReactFlow nodes={JSON.parse(functions[0]).instructions.split(' ').map((instructionName, index) => {
                return { id: `${index}`, data: { label: instructionName }, position: { x: 100, y: 100 * index } }
                  })} edges={[]}>
              {/* <Controls /> */}
            </ReactFlow> : ''
          } 
        </Box>
      </div>
    );
}
export default ContractContainer;