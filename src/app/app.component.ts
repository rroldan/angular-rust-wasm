import { Component } from '@angular/core';
import { add } from 'wasm-lib/pkg/wasm_lib_bg.wasm'

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})

export class AppComponent {
  title = 'angular-rust-wasm';
  total = 0;

  ngOnInit() {
    this.calc();
  }

  calc() {
    this.total = add(1, 1);
    }
}
