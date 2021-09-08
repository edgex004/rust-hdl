import { Component } from '@angular/core';
import { FormArray, FormBuilder, FormGroup } from '@angular/forms';

import data from '../assets/pcb_lists/pcb_list.json';
import interact from 'interactjs';
import { greet } from 'pcb-to-svg';

greet();

declare global {
  interface Window { dragMoveListener: any; }
}

var main_content_scale = 1.0;

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'pcb-decoupage';
  image_path = '';
  parts:any = [];


  sizeForm = this.formBuilder.group({
    width : "1200",
    height : "800",
    scale : "1.0"
  });
  constructor (              
    private formBuilder: FormBuilder,
    ){}



  ngOnInit(){
    this.parts = data.parts;
    this.image_path='assets/images/74LVC1G125SE-7.svg';

    var c = <HTMLCanvasElement> document.getElementById("testRawSVG");
    if (c) {
      c.innerHTML = '<svg height="100" width="100">\n    <circle cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red" />\n</svg>';
    }
  }

  async handleSizeFormSubmitted() {
    console.log("Width: " + this.sizeForm.value.width + " Height: " + this.sizeForm.value.height + " Scale: " + this.sizeForm.value.scale);
    if (isNaN(+this.sizeForm.value.width) ||
        isNaN(+this.sizeForm.value.height) ||
        isNaN(+this.sizeForm.value.scale) ||
        ( +this.sizeForm.value.width < 600 ) ||
        ( +this.sizeForm.value.width > 4000 ) ||
        ( +this.sizeForm.value.height < 600 ) ||
        ( +this.sizeForm.value.height > 4000 )||
        ( +this.sizeForm.value.scale < .25 ) ||
        ( +this.sizeForm.value.scale > 4 )
        ){
      return;
    }
    console.log("Setting width to: " + this.sizeForm.value.width)
    console.log("Setting height to: " + this.sizeForm.value.height)
    var c = document.getElementById("maincontent");
        if (c) {
          console.log("got main element.")

          c.style.width = this.sizeForm.value.width + "px";
          c.style.height = this.sizeForm.value.height + "px";
          c.style.transform = "scale(" + this.sizeForm.value.scale + ")";
          main_content_scale = this.sizeForm.value.scale;
        }

  }



}


// DVE: For now, all the interact.js stuff will be handled globally here. If it needs to make it's way inside a component, we can do that later.

// target elements with the "draggable" class
interact('.draggable')
.draggable({
  // enable inertial throwing
  inertia: true,
  // keep the element within the area of it's parent
  modifiers: [
    interact.modifiers.restrictRect({
      restriction: 'parent',
      endOnly: true
    })
  ],
  // enable autoScroll
  autoScroll: true,

  listeners: {
    // call this function on every dragmove event
    move: dragMoveListener,

    // call this function on every dragend event
    end (event) {
      var textEl = event.target.querySelector('p')

      textEl && (textEl.textContent =
        'moved a distance of ' +
        (Math.sqrt(Math.pow(event.pageX - event.x0, 2) +
                  Math.pow(event.pageY - event.y0, 2) | 0))
          .toFixed(2) + 'px')
    }
  }
})
window.dragMoveListener = dragMoveListener

function dragMoveListener (event: any) {
  var target = event.target,
      // keep the dragged position in the data-x/data-y attributes
      x = (parseFloat(target.getAttribute('data-x')) || 0) + event.dx / main_content_scale,
      y = (parseFloat(target.getAttribute('data-y')) || 0) + event.dy / main_content_scale;
  // translate the element
  target.style.webkitTransform =
  target.style.transform =
    'translate(' + x + 'px, ' + y + 'px)';
  target.setAttribute('data-x', x);
  target.setAttribute('data-y', y);
}