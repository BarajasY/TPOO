import { Component, onMount } from "solid-js";
import Chart from "chart.js/auto";
import { TodayPlotXHelper } from "../../types";
import "./plots.scss"

interface props {
  data: TodayPlotXHelper[],
  variant: 'Entrada' | 'Salida' | 'Combined',
  total: number
}

const LinePlot: Component<props> = (props:props) => {
  let containerRef: HTMLCanvasElement | undefined;

  onMount(() => {
    new Chart(containerRef!,
      {
        type: 'bar',
        data: {
          labels: props.data.map(d => d.name),
          datasets: [
            {
              label: props.variant,
              data: props.data.map(d => d.count)
            }
          ]
        }
      }
    )
  });

  return (
    <div class="line-plot-container" id='line-plot-container'>
      <canvas ref={containerRef}></canvas>
      <p class="total">Total: {props.total} {props.variant}(s)</p>
    </div>
  );
};

export default LinePlot;
