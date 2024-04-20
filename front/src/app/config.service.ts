import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';

/*
* This is the main handler for the http requests on the front end.
*/
@Injectable({
  providedIn: 'root'
})
export class ConfigService {
  url:any = 'api/';

  constructor(private http: HttpClient) { }

  getUser(user:string, password:string){
    return this.http.post<{status:String, message:string}>(this.url + 'login', {username: user, password: password}) //We send the username and password to the backend server
  }

  
}
