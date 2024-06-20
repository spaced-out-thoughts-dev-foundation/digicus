# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Instruction::ExitWithMessage do
  describe '#handle' do
    let(:instruction) { DTRCore::Instruction.new('exit_with_message', ['"Error message"'], nil, 0) }

    it 'returns the correct string' do
      expect(described_class.handle(instruction, [], [], false)).to eq('        panic! "Error message";')
    end
  end
end
