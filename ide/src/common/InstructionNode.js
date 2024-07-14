export function supportedInstructionToColor(supported_instruction) {
  if (supported_instruction == null || supported_instruction.category == null) {
    return 'white';
  }
  if (supported_instruction.category === "basic") {
    return 'orange';
  } else if (supported_instruction.category === "terminating") {
    return 'red';
  } else if (supported_instruction.category === "control_flow") {
    return 'turquoise';
  } else if (supported_instruction.category === "binary") {
    return 'lavender';
  } else if (supported_instruction.category === "logical") {
    return 'yellow';
  } else if (supported_instruction.category === "object") {
    return 'turquoise';
  } else {
    return 'white';
  }
};

export function determineInstructionNodeType(instructionName) {
  const instructionNodeTypeMap = {
    'add': 'binaryNode',
    'subtract': 'binaryNode',
    'multiply': 'binaryNode',
    'divide': 'binaryNode',
    'or': 'binaryNode',
    'and': 'binaryNode',
    'unary': 'unaryNode',
    'assign': 'assignNode',
    'evaluate': 'evaluateNode',
    'print': 'printNode',
    'instantiate_object': 'instantiateObjectNode',
    'exit_with_message': 'exitWithMessageNode',
    'return': 'returnNode',
    'goto': 'gotoNode',
    'jump': 'jumpNode',
    'try_assign': 'tryAssignNode',
    'end_of_iteration_check': 'endOfIterationCheckNode',
    'increment': 'incrementNode',
  };

  return instructionNodeTypeMap[instructionName] || 'unknownInstructionNode';
}

export function determineInstructionHeight(instructionNodeType, numInputs, hasAssign, instructionName, inputs) {
  let bonus_instructions = 0;
  if (instructionName === 'evaluate' && inputs[0].includes('.')) {
    bonus_instructions += 1;
  }

  if (instructionNodeType === 'binaryNode') {
    return 120;
  }

  return 65 + (20 * numInputs + bonus_instructions) + (hasAssign ? 50 : 0);
}