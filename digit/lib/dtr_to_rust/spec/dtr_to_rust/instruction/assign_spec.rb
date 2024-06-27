# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Instruction::Assign do
  describe '#handle' do
    let(:instruction) { DTRCore::Instruction.new('assign', ['5'], 'x', 0) }

    it 'returns the correct string' do
      expect(described_class.handle(instruction, 0, [], [], false, {}, {})).to eq('        let mut x = 5;')
    end
  end
end
