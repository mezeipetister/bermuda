import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Folder } from 'src/app/class/folder';

@Component({
  selector: 'app-folder',
  templateUrl: './folder.component.html',
  styleUrls: ['./folder.component.css']
})
export class FolderComponent implements OnInit {

  folders: Folder[] = [];

  constructor(private http: HttpClient) { }

  ngOnInit() {
    this.http.get<Folder[]>("/folder/all").subscribe((val) => this.folders = val);
  }

}
