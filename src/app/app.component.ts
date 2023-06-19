import { Component } from '@angular/core';
import { AppService } from './app.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
  providers:  [ AppService ]
})


export class AppComponent {
  title = 'angular-rust-wasm';
  total = 0;
  saludo = '';  
  
  constructor(private service: AppService) { }


  ngOnInit() {
    this.saludo = this.service.getHello();
    this.total = this.service.getResult();
  }
}
