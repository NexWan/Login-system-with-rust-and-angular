import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { ConfigService } from '../config.service';

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [],
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss'
})
export class HomeComponent implements OnInit {

  constructor(private route: ActivatedRoute, private router: Router, private config: ConfigService) { }

  ngOnInit(): void {
    console.log(sessionStorage.getItem('user'));
    
    if(sessionStorage.getItem('user') == null){
      alert('You need to login first');
      setTimeout(() => {
        this.router.navigate(['/']);
      }, 10); 
    }
    this.user = sessionStorage.getItem('user');
  }

  user: any;

  logout(){
    sessionStorage.removeItem('user');
    this.router.navigate(['/']);
  }

}
