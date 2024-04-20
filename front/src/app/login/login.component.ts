import { ActivatedRoute, Router } from '@angular/router';
import { Component } from '@angular/core';
import { ConfigService } from '../config.service';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [],
  templateUrl: './login.component.html',
  styleUrl: './login.component.scss'
})
export class LoginComponent {
user: any;
password: any;

  constructor(private router:Router, private route: ActivatedRoute, private config: ConfigService) { }

  login(){
    this.user = (<HTMLInputElement>document.getElementById('user')).value;
    this.password = (<HTMLInputElement>document.getElementById('password')).value;
    var res = this.config.getUser(this.user, this.password);
    res.subscribe((data)=>{
      if(data.status == 'success'){ //We get the status of the response of the backend server
        this.router.navigate(['home']); // If the status is success we redirect to the home page
      }else{
        alert(data.message);
      }
    });
    console.log(this.user, this.password);
  }

}
