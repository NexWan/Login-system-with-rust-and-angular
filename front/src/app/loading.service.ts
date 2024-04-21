import { Injectable } from '@angular/core';
import {Subject} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class LoadingService {
  private _loading = new Subject<boolean>();
  loading$ = this._loading.asObservable();

  setLoading(isLoading: boolean){
    this._loading.next(isLoading);
  }
  constructor() { }
}
