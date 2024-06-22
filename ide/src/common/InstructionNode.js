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