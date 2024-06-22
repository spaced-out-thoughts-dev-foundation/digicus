# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Aggregator::ScopeBlockAggregator do
  describe '.aggregate' do
    context 'when one scope that does not start at 0' do
      let(:instructions) do
        [
          ins('add', %w[foo bar], 'a', 1),
          ins('evaluate', %w[foobar a], 'b', 1),
          ins('return', ['b'], nil, 1)
        ]
      end

      let(:expected_scope_blocks) do
        [
          {
            block: instructions,
            scope: 1,
            decorated_value: 0
          }
        ]
      end

      it 'returns a single scope block' do
        expect(described_class.aggregate(instructions)).to eq(expected_scope_blocks)
      end
    end

    context 'when lesser to greater scope' do
      let(:instructions) do
        [
          ins('add', %w[foo bar], 'a', 1),
          ins('evaluate', %w[foobar a], 'b', 1),
          ins('return', ['b'], nil, 1),
          ins('add', %w[foo bar], 'a', 2),
          ins('evaluate', %w[foobar a], 'b', 2),
          ins('return', ['b'], nil, 2)
        ]
      end

      let(:expected_scope_blocks) do
        [
          {
            block: instructions[0..2],
            scope: 1,
            decorated_value: 0
          },
          {
            block: instructions[3..5],
            scope: 2,
            decorated_value: 1
          }
        ]
      end

      it 'returns multiple scope blocks' do
        expect(described_class.aggregate(instructions)).to eq(expected_scope_blocks)
      end
    end

    context 'when greater to lesser scope [unrealistic example]' do
      let(:instructions) do
        [
          ins('add', %w[foo bar], 'a', 2),
          ins('evaluate', %w[foobar a], 'b', 2),
          ins('return', ['b'], nil, 2),
          ins('add', %w[foo bar], 'a', 1),
          ins('evaluate', %w[foobar a], 'b', 1),
          ins('return', ['b'], nil, 1)
        ]
      end

      let(:expected_scope_blocks) do
        [
          {
            block: instructions[0..2],
            scope: 2,
            decorated_value: 0
          },
          {
            block: instructions[3..5],
            scope: 1,
            decorated_value: -1
          }
        ]
      end

      it 'returns multiple scope blocks' do
        expect(described_class.aggregate(instructions)).to eq(expected_scope_blocks)
      end
    end

    context 'when lesser to greater to lesser scope' do
      let(:instructions) do
        [
          ins('add', %w[foo bar], 'a', 1),
          ins('evaluate', %w[foobar a], 'b', 1),
          ins('return', ['b'], nil, 1),
          ins('add', %w[foo bar], 'a', 2),
          ins('evaluate', %w[foobar a], 'b', 2),
          ins('return', ['b'], nil, 2),
          ins('add', %w[foo bar], 'a', 1),
          ins('evaluate', %w[foobar a], 'b', 1),
          ins('return', ['b'], nil, 1)
        ]
      end

      let(:expected_scope_blocks) do
        [
          {
            block: instructions[0..2],
            scope: 1,
            decorated_value: 0
          },
          {
            block: instructions[3..5],
            scope: 2,
            decorated_value: 1
          },
          {
            block: instructions[6..8],
            scope: 1,
            decorated_value: 0
          }
        ]
      end

      it 'returns multiple scope blocks' do
        expect(described_class.aggregate(instructions)).to eq(expected_scope_blocks)
      end
    end

    context 'when greater to lesser to greater scope' do
      let(:instructions) do
        [
          ins('add', %w[foo bar], 'a', 2),
          ins('evaluate', %w[foobar a], 'b', 2),
          ins('return', ['b'], nil, 2),
          ins('add', %w[foo bar], 'a', 1),
          ins('evaluate', %w[foobar a], 'b', 1),
          ins('return', ['b'], nil, 1),
          ins('add', %w[foo bar], 'a', 2),
          ins('evaluate', %w[foobar a], 'b', 2),
          ins('return', ['b'], nil, 2)
        ]
      end

      let(:expected_scope_blocks) do
        [
          {
            block: instructions[0..2],
            scope: 2,
            decorated_value: 0
          },
          {
            block: instructions[3..5],
            scope: 1,
            decorated_value: -1
          },
          {
            block: instructions[6..8],
            scope: 2,
            decorated_value: 0
          }
        ]
      end

      it 'returns multiple scope blocks' do
        expect(described_class.aggregate(instructions)).to eq(expected_scope_blocks)
      end
    end

    context 'when scope decrements by more than 1' do
      let(:instructions) do
        [
          ins('add', %w[foo bar], 'a', 1),
          ins('evaluate', %w[foobar a], 'b', 2),
          ins('return', ['b'], nil, 3),
          ins('add', %w[foo bar], 'a', 1),
          ins('evaluate', %w[foobar a], 'b', 1),
          ins('return', ['b'], nil, 1)
        ]
      end

      let(:expected_scope_blocks) do
        [
          {
            block: instructions[0..0],
            scope: 1,
            decorated_value: 0
          },
          {
            block: instructions[1..1],
            scope: 2,
            decorated_value: 1
          },
          {
            block: instructions[2..2],
            scope: 3,
            decorated_value: 2
          },
          {
            block: instructions[3..5],
            scope: 1,
            decorated_value: 0
          }
        ]
      end

      it 'returns multiple scope blocks' do
        expect(described_class.aggregate(instructions)).to eq(expected_scope_blocks)
      end
    end

    context 'when scope increments by more than 1 [unrealistic example]' do
      let(:instructions) do
        [
          ins('add', %w[foo bar], 'a', 1),
          ins('evaluate', %w[foobar a], 'b', 2),
          ins('return', ['b'], nil, 3),
          ins('add', %w[foo bar], 'a', 1),
          ins('evaluate', %w[foobar a], 'b', 3),
          ins('return', ['b'], nil, 3)
        ]
      end

      let(:expected_scope_blocks) do
        [
          {
            block: instructions[0..0],
            scope: 1,
            decorated_value: 0
          },
          {
            block: instructions[1..1],
            scope: 2,
            decorated_value: 1
          },
          {
            block: instructions[2..2],
            scope: 3,
            decorated_value: 2
          },
          {
            block: instructions[3..3],
            scope: 1,
            decorated_value: 0
          },
          {
            block: instructions[4..5],
            scope: 3,
            decorated_value: 2
          }
        ]
      end

      it 'returns multiple scope blocks' do
        expect(described_class.aggregate(instructions)).to eq(expected_scope_blocks)
      end
    end

    context 'when multiple scope blocks are interleaved' do
      let(:instructions) do
        [
          ins('add', %w[foo bar], 'a', 1),
          ins('evaluate', %w[foobar a], 'b', 2),
          ins('return', ['b'], nil, 3),
          ins('add', %w[foo bar], 'a', 1),
          ins('evaluate', %w[foobar a], 'b', 1),
          ins('return', ['b'], nil, 1),
          ins('add', %w[foo bar], 'a', 2),
          ins('evaluate', %w[foobar a], 'b', 3),
          ins('return', ['b'], nil, 101),
          ins('return', ['b'], nil, 2)
        ]
      end

      let(:expected_scope_blocks) do
        [
          {
            block: instructions[0..0],
            scope: 1,
            decorated_value: 0
          },
          {
            block: instructions[1..1],
            scope: 2,
            decorated_value: 1
          },
          {
            block: instructions[2..2],
            scope: 3,
            decorated_value: 2
          },
          {
            block: instructions[3..5],
            scope: 1,
            decorated_value: 0
          },
          {
            block: instructions[6..6],
            scope: 2,
            decorated_value: 1
          },
          {
            block: instructions[7..7],
            scope: 3,
            decorated_value: 2
          },
          {
            block: instructions[8..8],
            scope: 101,
            decorated_value: 3
          },
          {
            block: instructions[9..9],
            scope: 2,
            decorated_value: 1
          }
        ]
      end

      it 'returns multiple scope blocks' do
        expect(described_class.aggregate(instructions)).to eq(expected_scope_blocks)
      end
    end
  end
end
