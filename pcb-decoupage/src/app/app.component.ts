import { Component } from '@angular/core';
// // With the Tauri API npm package:
// import * as tauri from '@tauri-apps/api/tauri'
// // With the Tauri global script, enabled when `tauri.conf.json > build > withGlobalTauri` is set to true:
// const invoke = window.__TAURI__.invoke

// // Invoke the command
// invoke('my_custom_command')

import data from '../assets/pcb_lists/pcb_list.json';



@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'pcb-decoupage';
  image_path = '';
  parts:any = [];

  constructor (){}
  
  ngOnInit(){
    this.parts = data.parts;
    this.image_path='assets/images/74LVC1G125SE-7.svg';
  }

  handleSaveJsonClicked(){
    // var draggables = document.getElementsByClassName("draggable");
    // for (var i = 0; i < draggables.length; i++) {
    //   console.log(draggables[i].id);
    // }
  }
}

