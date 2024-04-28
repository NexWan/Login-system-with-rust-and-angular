import { ChangeDetectorRef, Component } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { ConfigService } from '../../config.service';
import { LoadingService } from '../../loading.service';
import { NgIf } from '@angular/common';

@Component({
  selector: 'app-header',
  standalone: true,
  imports: [NgIf],
  templateUrl: './header.component.html',
  styleUrl: './header.component.scss'
})
export class HeaderComponent {
  title: string = 'Frontend';
  constructor(private router:Router, private route: ActivatedRoute, private config: ConfigService, private loadingService:LoadingService, private cdr: ChangeDetectorRef) {
  }

  
}
