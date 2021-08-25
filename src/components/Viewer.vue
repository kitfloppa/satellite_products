<template lang="pug">
    div(id='vie')
        div(class='slider')
            div(class='slider_wrapper')
                div(class='slider_items')
                    div(class='slider_item')
                        img(id='photo1')
                    div(class='slider_item')
                        img(id='photo2')
                        img(id='dt')
</template>

<script>
/* eslint-disable */
import * as slider from './slider'

export default {
    name: 'Viewer',
    created: function() {
        document.addEventListener('DOMContentLoaded', function () {
            var sl = new slider.SimpleAdaptiveSlider('.slider', {interval: 10000})
        })
    }
}
</script>

<style>
#vie {
    position: fixed;
    right: 0;
    top: 0;
    width: 40%;
    height: 100%;
    background: black;
    overflow: hidden;
}

img {
    width: 700px;
    margin: 120px 35px;
    position: relative;
}

#dt {
    bottom: 50%;
    z-index: 1;
}

.slider {
  position: relative;
  overflow: hidden;
  height: 100%;
  margin-left: auto;
  margin-right: auto;
}

.slider_wrapper {
  position: relative;
  overflow: hidden;
  background-color: #000;
}

.slider_items {
  display: flex;
  transition: transform 0.5s ease;
}

.slider_item {
  flex: 0 0 100%;
  max-width: 100%;
  position: relative;
}

.slider_item2 {
  flex: 0 0 100%;
  max-width: 100%;
  position: relative;
}

.slider_indicators {
  position: absolute;
  left: 0;
  right: 0;
  bottom: 10px;
  display: flex;
  justify-content: center;
  padding-left: 0;
  margin: 0 15%;
  list-style: none;
  user-select: none;
}

.slider_indicator {
  flex: 0 1 auto;
  width: 30px;
  height: 4px;
  margin-right: 3px;
  margin-left: 3px;
  background-color: rgba(255, 255, 255, 0.5);
  background-clip: padding-box;
  border-top: 10px solid transparent;
  border-bottom: 10px solid transparent;
  cursor: pointer;
}

.slider_indicator_active {
  background-color: #fff;
}
</style>
