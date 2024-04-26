import * as d3 from "d3";
import { useEffect, useRef, useMemo } from "react";

const Block = () => {
    const ticks = useMemo(() => {
        const xScale = d3.scaleLinear()
          .domain([0, 100])
          .range([10, 290])
        return xScale.ticks()
          .map(value => ({
            value,
            xOffset: xScale(value)
          }))
      }, [])
      return (
        <svg>
          <path
            d="M 9.5 0.5 H 290.5"
            stroke="currentColor"
          />
          {ticks.map(({ value, xOffset }) => (
            <g
              key={value}
              transform={`translate(${xOffset}, 0)`}
            >
              <line
                y2="6"
                stroke="currentColor"
              />
              <text
                key={value}
                style={{
                  fontSize: "10px",
                  textAnchor: "middle",
                  transform: "translateY(20px)"
                }}>
                { value }
              </text>
            </g>
          ))}
        </svg>
      )
};

export default Block;