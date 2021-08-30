import { Component } from '@angular/core';

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

}

