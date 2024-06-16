# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Instruction::InitializeUDT do
  describe '#handle' do
    it 'returns the correct Rust code when assign exists' do
      instruction = {
        instruction: 'initialize_udt',
        inputs: %w[State 0 0],
        assign: '1_METHOD_CALL_ARG_0',
        scope: 0
      }

      expect(described_class.handle(instruction)).to eq('        let mut 1_METHOD_CALL_ARG_0 = State { 0 0 };')
    end
  end
end
