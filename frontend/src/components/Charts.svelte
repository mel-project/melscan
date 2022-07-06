<script lang="ts">
    import type { PoolKey } from "@utils/types";


    export let pool_key: PoolKey;
    console.log(`{left: "{pool_key.left}", right: "{pool_key.right}"}`)
    var pool_key = JSON.parse(`{"left": "{pool_key.left}", "right": "{pool_key.right}"}`)
    var denom = pool_key.right
    var denom_left = pool_key.left
    var last_item = JSON.parse(`{last_item|json}`)
    var DateTime = luxon.DateTime
    var Duration = luxon.Duration
    console.info(`Denom: ${denom} \nLast Item: ${JSON.stringify(last_item)}`)
  
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
  
        $(chart.ctx.canvas).css('cursor', 'wait')
  
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
        $(chart.ctx.canvas).css('cursor', 'pointer')
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
          //   autoSkipPadding: 50,
          //   maxRotation: 0
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
          enabled: true,Charts
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
        responsive: true,
        maintainAspectRatio: false,
      };
  
      return options
    }
    function chartSpec(pooldata, y_axis, label, options) {
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
          }].concat(pool_key.left == "ERG" && pool_key.right == "MEL" ? [{
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
  
    async function initChart() {
  
    }
    async function main() {
      //pooldata is a writable array of immutable pooldata objects
      let pooldata = await getPoolData(denom, 0, last_item.height)
  
      let fetchFunction = debounce(updateData(pooldata), 200);
  
      let y_axis = 'price'
      let options = chartOptions(pooldata, y_axis, fetchFunction);
      console.info('pooldata: ', pooldata)
  
  
      let legend_label = createLegendLabel(pool_key, 'Liquidity')
      var spec = chartSpec(pooldata, y_axis, legend_label, options)
  
      let ctx = document.getElementById('chart').getContext('2d')
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
      handlers.loadLiquidity = () => {
        Object.assign(chart.options, chartOptions(pooldata, 'liquidity', fetchFunction))
        let dataset = chart.data.datasets[0]
  
        dataset.parsing.yAxisKey = 'liquidity'
        dataset.label = createLegendLabel(pool_key, 'Liquidity')
        chart.update()
      }
      handlers.loadPrice = () => {
        Object.assign(chart.options, chartOptions(pooldata, 'price', fetchFunction))
        let dataset = chart.data.datasets[0]
  
        dataset.parsing.yAxisKey = 'price'
        dataset.label = createLegendLabel(pool_key, 'Price')
        chart.update()
      }
      handlers.loadBeforeNow = (time) => {
        return () => {
          let timescale = { min: DateTime.now().minus(time).toMillis(), max: DateTime.now().toMillis() }
          chart.zoomScale('x', timescale, 'easeInExpo')
          setTimeout(() => fetchFunction({ chart }), 500)
        }
      }
      handlers.loadLastDay = handlers.loadBeforeNow({ day: 1 })
      handlers.loadLastWeek = handlers.loadBeforeNow({ week: 1 })
      handlers.loadLastMonth = handlers.loadBeforeNow({ month: 1 })
      handlers.loadAllTime = () => {
        chart.resetZoom('easeOutQuad')
        setTimeout(() => fetchFunction({ chart }), 500)
      }
  
      return { pooldata, handlers, chart }
    }
    var melscan = {}
    var handlers;
    var chart;
    var tooltip;
    main().then(i => {
      Object.assign(melscan, i)
      handlers = melscan.handlers
      chart = melscan.chart
      tooltip = chart.options.plugins.tooltip
      handlers.loadPrice()
    })
</script>
