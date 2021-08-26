import { Component } from '@angular/core';
// import { DomSanitizer} from '@angular/platform-browser';



@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'pcb-decoupage';
  // eulaContent = {};

  // constructor (private sanitizer: DomSanitizer){}
  
  // ngOnInit(){
  //   // <img src="assets/images/74LVC1G125SE-7_lr_ud.svg">

  //   fetch('assets/images/74LVC1G125SE-7_lr_ud.svg').then(res => res.text()).then(data => {
  //     this.eulaContent = this.sanitizer.bypassSecurityTrustHtml(data);
  //     var svg = document.getElementsByTagName("svg")[0];
  //     var bbox = svg.getBBox({clipped:true,fill:false,markers:true,stroke:false});
  //     var viewBox = [bbox.x, bbox.y, bbox.width, bbox.height].join(" ");
  //     svg.setAttribute("viewBox", viewBox);
    
  //   })

  // }
  
}

