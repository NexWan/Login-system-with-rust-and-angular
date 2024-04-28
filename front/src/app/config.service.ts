import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import {LoadingService} from "./loading.service";

/*
* This is the main handler for the http requests on the front end.
*/
@Injectable({
  providedIn: 'root'
})
export class ConfigService {
  url:any = 'api/';

  constructor(private http: HttpClient, private loadingService:LoadingService) { }

  async getUser(user:string, password:string){
    this.loadingService.setLoading(true)
    let x = this.http.post<{status:String, message:string}>(this.url + 'login', {username: user, password: password}) //We send the username and password to the backend server
    return x;
  }

  async register(user:string, password:string){
    this.loadingService.setLoading(true)
    let x = this.http.post<{status:String, message:string}>(this.url + 'register', {username: user, password: password}) //We send the username and password to the backend server
    return x;
  }

  
}
