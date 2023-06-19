import { Injectable } from '@angular/core';
import { add } from '../../wasm-lib/pkg/wasm_lib_bg.wasm';

@Injectable()
export class AppService {
    constructor() { }
    getHello(): string {
        return 'Hello World!';
    }
    getResult(): number {
        let result = 0;
        result = add(1, 1);
   
    return result;
    }
}