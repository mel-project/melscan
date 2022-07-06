<script lang="ts">
    import {debounce} from 'lodash/debounce';
    import type { BubbleDataPoint,  ChartConfiguration,  ChartTypeRegistry, ScatterDataPoint} from 'chart.js';
    import {Chart, UpdateModeEnum} from 'chart.js';
    import {Denom} from "@utils/types";
    import type { PoolKey } from "@utils/types";
    import * as luxon from "luxon"
    import { onMount } from 'svelte';
    import zoomPlugin from 'chartjs-plugin-zoom';
    import 'chartjs-adapter-luxon';
    
    Chart.register(zoomPlugin);

    export let pool_key: PoolKey;
    export let last_item;
    let chart_canvas: HTMLCanvasElement;
    var denom = pool_key.right
    var denom_left = pool_key.left
    var DateTime = luxon.DateTime
    var Duration = luxon.Duration
    let pooldata; 
    let chart_loading;


    onMount(async ()=>{
        //pooldata is a writable array of immutable pooldata objects
        pooldata = await getPoolData(denom, 0, last_item.height)
    })
  
    let fetchFunction = debounce(updateData(pooldata), 200);

    let y_axis = 'price'
    let options = chartOptions(pooldata, y_axis, fetchFunction);
    console.info('pooldata: ', pooldata)


    let legend_label = createLegendLabel(pool_key, 'Liquidity')
    var spec = chartSpec(pooldata, y_axis, legend_label, options)

    let ctx = chart_canvas.getContext('2d')
    let chart = new Chart(ctx, spec)
    console.info("Chart Specification: ", spec)

    chart.update()
    // setInterval(()=>{
    //   chart.data.datasets[0].data = pooldata.map( i => [i.date, i.liquidity])
    //   chart.update()
    // },1000)
    var handlers = {}
    // handlers.toggleDataVisibility = (index, update = true) => {
    //   if(chart.getDataVisibility(index)){
    //     chart.setDatasetVisibility(index, false)
    //     chart.hide(index)
    //   }
    //   else {
    //     chart.setDatasetVisibility(index, true)
    //     chart.show(index)
    //   }
    // }
  let loadLiquidity = () => {
    Object.assign(chart.options, chartOptions(pooldata, 'liquidity', fetchFunction))
    let dataset = chart.data.datasets[0]

    dataset.parsing['yAxisKey'] = 'liquidity'
    dataset.label = createLegendLabel(pool_key, 'Liquidity')
    chart.update()
  }
  let loadPrice = () => {
    Object.assign(chart.options, chartOptions(pooldata, 'price', fetchFunction))
    let dataset = chart.data.datasets[0]

    dataset.parsing['yAxisKey'] = 'price'
    dataset.label = createLegendLabel(pool_key, 'Price')
    chart.update()
  }
  let loadBeforeNow = (time) => {
    return () => {
      let timescale = { min: DateTime.now().minus(time).toMillis(), max: DateTime.now().toMillis() }
      chart.zoomScale('x', timescale, UpdateModeEnum.resize)
      setTimeout(() => fetchFunction({ chart }), 500)
    }
  }
  let loadLastDay = loadBeforeNow({ day: 1 })
  let loadLastWeek = loadBeforeNow({ week: 1 })
  let loadLastMonth = loadBeforeNow({ month: 1 })

  let loadAllTime = () => {
    chart.resetZoom(UpdateModeEnum.show);
    setTimeout(() => fetchFunction({ chart }), 500)
  }
  
    async function getPoolData(denom, lower, upper) {
      let url = `/raw/pooldata/${pool_key.left}/${pool_key.right}/${lower}/${upper}`
      console.log('Request:', url)
      let poolData = await fetch(url)
      // replace the date string with a date object
      // create immutable objects with freeze
      return poolData.map(i => (Object.assign({}, i, { date: i.date * 1000 }))).map(Object.freeze)
    }
  
    function getDataRange(handleTicks) {
      return (value, index, ticks) => {
        if (ticks.length - 1 === index) {
          handleTicks(ticks)
        }
        return value
      }
    }
  
    function uniq(a, property) {
      var seen = {};
      return a.filter((item) => {
        let prop = property(item)
        return seen.hasOwnProperty(prop) ? false : (seen[prop] = true);
      });
    }
    function updateData(pooldata) {
      return async ({ chart }) => {
        console.log("****** UPDATE DATA")
        let { min, max } = chart.scales.x
        let [lower, upper] = findBlockRange(pooldata, min, max)
  
        chart_loading = true;
  
        let new_data = await getPoolData(denom, pooldata[lower].height, pooldata[upper].height)
        new_data.push(...pooldata.filter((d) => d.height < new_data[0].height ||
          d.height > new_data[new_data.length - 1].height))
        new_data = new_data.sort((i, j) => i.height - j.height);
        let uniq_data = uniq(new_data, (i) => i.height)
        pooldata.length = 0
        //console.log(pooldata);
        pooldata.push(...new_data);
        // console.log(new_data);
  
        chart.stop()
        chart.update('none')
        chart_loading = false;
      }
    }
    function findBlockRange(pooldata, minTime, maxTime) {
      if (pooldata.length == 0)
        return [0, pooldata.length - 1]
      else {
        let upper = pooldata.findIndex(data => data.date >= maxTime)
        if (upper < 0) {
          upper = pooldata.length - 1
        }
        let lower = findIndexReverse(pooldata, data => data.date <= minTime)
        return [lower, upper]
      }
    }
    function findIndexReverse(list, predicate) {
      for (let i = list.length - 1; i >= 0; i--) {
        if (predicate(list[i])) return i
      }
    }
    function createLegendLabel(pool_key, info) {
      return `${pool_key.left}/${pool_key.right} ${info}`
    }
    function chartOptions(pooldata, pooldata_key, fetchFunction) {
      const scales = {
        x: {
          type: 'time',
          // ticks: {
          //   autoSkip: true,
          //   autoSkipPadding: 50,Mel
          // },
          time: {
            displayFormats: {
              hour: 'hh:mm',
              minute: 'hh:mm',
              second: 'hh:mm:ss'
            }
          },
          min: pooldata[0].date,
          max: pooldata[pooldata.length - 1].date,
        },
        y: {
          //min: 0,
          // max: Math.max(...pooldata.map(i => i[pooldata_key])) * 1.20,
          display: true,
        },
      };
      const tooltipOptions = {
        callbacks: {
          label: function (item) {
            return item.dataset.label + ": " + item.formattedValue
          },
  
        }
      }
      const zoomOptions = {
        limits: {
          y: { min: 0, minRange: 50 },
          x: { min: pooldata[0].date, max: Date.now() },
        },
        pan: {
          enabled: true,
          mode: 'x',
          overScaleMode: 'xy',
          onPan: fetchFunction
        },
        zoom: {
          wheel: {
            enabled: true,
          },
          pinch: {
            enabled: true
          },
          drag: {
            enabled: true,
          },
          mode: 'x',
          onZoom: fetchFunction
        }
      };
      let options = {
        scales,
        // tooltips: {
        //   intersect: false,
        //   mode: 'index',
        // },
        plugins: {
          zoom: zoomOptions,
          tooltip: tooltipOptions
        },
        interaction: {
          intersect: false,
          mode: 'index',
          axis: 'x'
        },
        transitions: {
          zoom: {
            animation: {
              duration: 500,
              easing: 'easeOutQuad'
            }
          }
        },
        point: {
          radius: 0,
          pointStyle: 'rect',
          hoverRadius: 10,
        },
        maintainAspectRatio: false,
      };
  
      return options
    }
    function chartSpec(pooldata, y_axis, label, options):  ChartConfiguration<keyof ChartTypeRegistry, (number | ScatterDataPoint | BubbleDataPoint)[], unknown>{
      return {
        type: 'line',
        options,
        data: {
          datasets: [{
            label,
            data: pooldata,
            fill: 'origin',
            borderWidth: 1,
            radius: 0,
            parsing: {
              yAxisKey: y_axis,
              xAxisKey: 'date'
            },
            backgroundColor: '#d8eae566',
            borderColor: '#006e54',
          }].concat(pool_key.left == Denom.ERG && pool_key.right == Denom.MEL ? [{
            label: "1 DOSC",
            data: pooldata,
            borderWidth: 1,
            borderDash: [10, 5],
            radius: 0,
            parsing: {
              yAxisKey: 'ergs_per_mel',
              xAxisKey: 'date'
            },
            borderColor: '#640125',
          }] : []),
        },
      }
  
    }
  
     
  
    // var melscan = {}
    // var handlers;
    // var chart;
    // var tooltip;
    // main().then(i => {
    //   Object.assign(melscan, i)
    //   handlers = melscan.handlers
    //   chart = melscan.chart
    //   tooltip = chart.options.plugins.tooltip
    //   handlers.loadPrice()
    // })
</script>

<template>
    <canvas bind:this={chart_canvas}></canvas>
</template>