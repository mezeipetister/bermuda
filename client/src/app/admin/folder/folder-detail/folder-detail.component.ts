import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { ActivatedRoute } from '@angular/router';
import { Folder } from 'src/app/class/folder';

@Component({
  selector: 'app-folder-detail',
  templateUrl: './folder-detail.component.html',
  styleUrls: ['./folder-detail.component.css']
})
export class FolderDetailComponent implements OnInit {

  id: string = this.route.snapshot.paramMap.get("id");
  folder: Folder = new Folder();

  constructor(private http: HttpClient, private route: ActivatedRoute) { }

  ngOnInit() {
    this.http.get<Folder>("/folder/" + this.id).subscribe((val) => this.folder = val);
  }

}
