<script setup lang="ts">
import * as am5 from '@amcharts/amcharts5';
import am5themes_Animated from '@amcharts/amcharts5/themes/Animated';
import am5themes_Dark from '@amcharts/amcharts5/themes/Dark';
import * as am5xy from '@amcharts/amcharts5/xy';

import { computed, onBeforeUnmount, onMounted, ref, type PropType } from 'vue';

export interface ScheduledTask {
  machine: number,
  name: string,
  start: Date,
  end: Date,
  tooltip?: string
}

const props = defineProps({
  tasks: {
    type: Array as PropType<ScheduledTask[]>,
    required: true
  },
  machineNameFormatter: {
    type: Function as PropType<(machine: number) => string>,
    required: false,
    default: (machine: number) => `Pracownik ${machine}`
  }
});

const labels = computed(() => {
  return Array.from(new Set(props.tasks.map(task => task.machine)))
    .sort((a, b) => a - b);
});


const chartdiv = ref();
let root: am5.Root;
onMounted(() => {
  root = am5.Root.new(chartdiv.value);
  root.dateFormatter.setAll({
    dateFormat: "yyyy-MM-dd HH:mm",
    dateFields: ["valueX", "openValueX"]
  });
  root.setThemes([
    am5themes_Animated.new(root),
    am5themes_Dark.new(root)]);

  const chart = root.container.children.push(am5xy.XYChart.new(root, {
    panX: false,
    panY: false,
    wheelX: "panX",
    wheelY: "zoomX",
    paddingLeft: 0,
    paddingTop: 100,
    paddingBottom: 100,
    layout: root.verticalLayout
  }));
  // Define data
  const colors = chart.get("colors")!;
  let idx = 0;
  const tasks = props.tasks;
  tasks.sort((a, b) => a.start.getTime() - b.start.getTime());
  const data = tasks.map(task => {
    const start = task.start.getTime();
    const end = task.end.getTime();
    return {
      machine: props.machineNameFormatter(task.machine),
      start: start,
      end: end,
      task: task.name,
      duration: `${(end - start) / 1000 / 60} min`,
      columnSettings: {
        fill: colors.getIndex(idx++)
      }
    }
  });

  const yAxis = chart.yAxes.push(
    am5xy.CategoryAxis.new(root, {
      categoryField: "machine",
      renderer: am5xy.AxisRendererY.new(root, {
        minGridDistance: 1,
        inversed: true
      })
    })
  );
  yAxis.data.setAll(labels.value.map(x => ({ machine: props.machineNameFormatter(x) })));

  let cursor = chart.set("cursor", am5xy.XYCursor.new(root, {
    behavior: "zoomX"
  }));
  cursor.lineY.set("visible", false);

  const xAxis = chart.xAxes.push(
    am5xy.DateAxis.new(root, {
      baseInterval: { timeUnit: "minute", count: 1 },
      renderer: am5xy.AxisRendererX.new(root, {
        strokeOpacity: 0.1,
        minorGridEnabled: true,
        minGridDistance: 200,
      }),
      tooltip: am5.Tooltip.new(root, {})
    }),
  );

  const series = chart.series.push(am5xy.ColumnSeries.new(root, {
    xAxis: xAxis,
    yAxis: yAxis,
    openValueXField: "start",
    valueXField: "end",
    categoryYField: "machine",
  }));
  series.columns.template.setAll({
    cornerRadiusBL: 5,
    cornerRadiusBR: 5,
    cornerRadiusTL: 5,
    cornerRadiusTR: 5
  });


  series.columns.template.setAll({
    templateField: "columnSettings",
    strokeOpacity: 0,
    fillOpacity: 0.5,

    tooltipText: "[bold]{task}[/]:\n\n[bold]Start:[/] {openValueX}\n[bold]Koniec:[/] {valueX}\n[bold]Czas trwania:[/] {duration}"
  });

  series.columns.template.set("interactive", true);

  series.columns.template.states.create("hover", {
    fillOpacity: 1.0
  });

  const scrollbar = chart.set("scrollbarX", am5xy.XYChartScrollbar.new(root, {
    orientation: "horizontal",
    height: 30
  }));

  const sbxAxis = scrollbar.chart.xAxes.push(am5xy.DateAxis.new(root, {
    baseInterval: {
      timeUnit: "minute",
      count: 1
    },
    renderer: am5xy.AxisRendererX.new(root, {
      minorGridEnabled: true,
      minGridDistance: 70
    })
  }));

  const sbyAxis = scrollbar.chart.yAxes.push(
    am5xy.CategoryAxis.new(root, {
      categoryField: "machine",
      renderer: am5xy.AxisRendererY.new(root, {
        minGridDistance: 1,
        inversed: true
      })
    })
  );
  sbyAxis.data.setAll(labels.value.map(x => ({ machine: props.machineNameFormatter(x) })));

  const sbSeries = scrollbar.chart.series.push(am5xy.ColumnSeries.new(root, {
    xAxis: sbxAxis,
    yAxis: sbyAxis,
    openValueXField: "start",
    valueXField: "end",
    categoryYField: "machine"
  }));

  series.data.setAll(data);
  sbSeries.data.setAll(data);

  let chartHeight = 50 * labels.value.length + xAxis.height() + 120 + 200;
  chart.root.dom.style.height = chartHeight + "px";

  series.events.once("datavalidated", () => {
    const firstDate = Math.min(...props.tasks.map(task => task.start.getTime()));
    xAxis.zoomToDates(new Date(firstDate), new Date(firstDate + 1000));
  });
});
onBeforeUnmount(() => {
  root.dispose();
});
</script>

<template>
  <div class="w-100" style="height: 200px" ref="chartdiv">
  </div>
</template>

<style scoped></style>
