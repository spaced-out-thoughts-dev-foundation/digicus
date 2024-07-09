# frozen_string_literal: true

require './spec/spec_helper'

RSpec.describe DTRCore::InstructionValidator do
  context 'when invalid instruction' do
    it 'returns false when unrecognized instruction' do
      instruction = DTRCore::Instruction.new('invalid_instruction', nil, nil, 0, 0)

      expect(described_class.new(instruction).valid?).to be(false)
    end

    it 'returns raises when missing instruction (whole instruction, not just the name)' do
      expect { described_class.new(nil).valid? }.to raise_error('Missing Instruction.')
    end

    it 'returns raises when missing instruction (the name)' do
      instruction = DTRCore::Instruction.new(nil, nil, nil, nil, 0)

      expect { described_class.new(instruction).valid? }.to raise_error('Instruction name missing.')
    end

    it 'returns raises when missing scope' do
      instruction = DTRCore::Instruction.new('assign', nil, nil, nil, 0)

      expect { described_class.new(instruction).valid? }.to raise_error('Instruction missing scope.')
    end

    it 'returns false when scope is less than 0' do
      instruction = DTRCore::Instruction.new('assign', nil, nil, -1, 0)

      expect(described_class.new(instruction).valid?).to be(false)
    end

    context 'when logical operation' do
      it "returns false when instruction is 'and' and is missing required two inputs" do
        instruction = DTRCore::Instruction.new('and', nil, nil, 0, 0)

        expect(described_class.new(instruction).valid?).to be(false)
      end

      it "returns false when instruction is 'or' and is missing required two inputs" do
        instruction = DTRCore::Instruction.new('or', ['a'], nil, 0, 0)

        expect(described_class.new(instruction).valid?).to be(false)
      end
    end

    context 'when binary operation' do
      it "returns false when instruction is 'add' and is missing required two inputs" do
        instruction = DTRCore::Instruction.new('add', nil, nil, 0, 0)

        expect(described_class.new(instruction).valid?).to be(false)
      end

      it "returns false when instruction is 'subtract' and is missing required two inputs" do
        instruction = DTRCore::Instruction.new('subtract', ['a'], nil, 0, 0)

        expect(described_class.new(instruction).valid?).to be(false)
      end

      it "returns false when instruction is 'multiply' and is missing required two inputs" do
        instruction = DTRCore::Instruction.new('multiply', nil, nil, 0, 0)

        expect(described_class.new(instruction).valid?).to be(false)
      end

      it "returns false when instruction is 'divide' and is missing required two inputs" do
        instruction = DTRCore::Instruction.new('divide', %w[a c d], nil, 0, 0)

        expect(described_class.new(instruction).valid?).to be(false)
      end
    end
  end
end
